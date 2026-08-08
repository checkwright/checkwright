# SPEC amendment: trajectory-prune-on-completion

`TRAJECTORY.md` becomes what its own header already says it is: **a record of
what still directs**, not a history of what was decided. The completion-time
half of its authoring contract lands in its header, `§The closed rulings` is
dispositioned against that contract, and one existing gate is extended so the
two idioms this ruling retires cannot come back silently.

**Operator-ruled 2026-08-08**, as the fourth unit of `install-profile-seam` —
the operator's own widening of a three-unit cut. Two triggers are the whole
disposition rule: **spent ⇒ delete outright**, **stale ⇒ correct in place**.

**This unit reverses nothing, and that is worth stating once because the
never-reverse rule would otherwise stall a build session.** The file's header
(§Where the grounds live) already rules one owner per fact and pointers rather
than restatement; §The closed rulings had drifted from it. The 2026-08-08
pruning ruling recorded in that section sanctions the practice and
**deliberately withholds two things — who may prune, and on what event —
stating in terms that "nothing here licenses a session to prune today."** The
operator's directive supplies exactly those two. This unit discharges a
recorded non-settlement; it does not author licence.

**The append-never-strike precedent is retired, and D1 must land the retirement
explicitly.** It rests on the claim that striking a clause "would destroy the
evidence of what was believed." The file refutes itself on this within one
section: the 2026-08-08 ruling holds that git history carries obsolete changes
*and the motivation behind them*, while the 2026-08-07 correction holds that
striking destroys the evidence of belief. Both cannot be true of one git
history. The precedent falls for resting on a false ground, not because a
competing interest was outweighed — there was no interest at risk. A build that
prunes without landing the retirement leaves the next session reading the
append-never-strike text and preserving precisely what this ruling deletes.

## What changes

**D1 — `TRAJECTORY.md`'s header gains the completion-time contract, in the
three sections that already carry the authoring-time one.** `{design-bearing}`

*§How to read a ruling recorded here* gains the distinction the never-reverse
rule needs in order to not block this work: **retiring a spent ruling is not
reversing it.** A ruling whose subject is finished directed something that
already happened; deleting the record changes no decision. Reversal — making a
closed ruling no longer the rule — stays operator-class and unchanged. The
existing *re-verify facts against the tree* duty is dropped, because D2 removes
the staleness that duty existed to absorb; a reader who must re-verify the file
before trusting it is reading a file that has already failed.

*§Who may record a ruling here* gains the symmetric authority: **the session
that may record may retire.** Recording and retirement are the two ends of one
authority precisely because neither decides anything — the authority is the
ruling's existence in the one case and its discharge in the other, never the
session's role. What a retirement may never do is re-decide.

*§Where the grounds live* gains the completion-time half the section never had —
two triggers, and they are the whole rule:

- **A ruling whose subject is finished is deleted outright.** Not distilled to a
  line, and not annotated as finished. Git history holds the obsolete text and
  the motivation behind it; the cost of going there to retrieve it is the
  accepted cost.
- **A fact that has aged is corrected where it stands.** A correction is never
  appended, and a superseded sentence is never left standing beside its
  correction — two readings of one fact is the defect, whichever of them is
  labelled current.

Plus one authoring convention that makes the next application cheap: **a ruling
able to name its own discharge event says so in its own text**, so the session
that meets the event deletes rather than judges. This is prose, not a
machine-readable declaration — the declaration is
`ruling-record-condition-staleness-probe`'s deliverable and is not built here
(D9).

**Two constraints on how D1 is worded, both load-bearing.** The contract must
be statable without using any marker D6 configures, and the merged text must
leave `check-manifest-temporal` green — verify by running it, not by reading.
And the contract is stated in the file's own header rather than in a kit SPEC
because the file is repo-root-governed with no owning kit, and its header is
already where its three authoring rules live.

**D2 — `§The closed rulings` is dispositioned entry by entry against
D1.** `{design-bearing}`

This is the roster, and it is the design judgment the unit owns. Rulings are
anchored by their opening bold phrase and date, never by line number. **Nine
keep, the rest go.**

*Keep, unchanged — each still directs and each has no other durable home:*

1. **The substrate language is Rust — 2026-08-02.** Registered with its pointer
   to gate-sdk/SPEC.md §The decisions this substrate already closed, which is
   the shape §Where the grounds live asks for.
2. **A bash portability floor was costed and rejected.** Same shape; closed, not
   deferred.
3. **git is the sole runtime dependency.** Bounds what a governed surface may
   claim about the *build* versus the *run*.
4. **Building from vendored crate source at install time is void.** Self-
   justifying as a keep: it exists so the next session reaching for the cheap
   answer finds it already costed.
5. **The interpreter policy.** Two standing obligations bind every unit touching
   the install path. Both slugs it names — `powershell-installer-surface`,
   `install-step-relocation` — verified live in the queue.
6. **Opacity is taken on deliberately, and it carries an obligation.** Bounds
   prose exactly: *verified against a published digest*, never *reproducible*.
   `tarball-build-attestation` verified live.
7. **The release policy's security-or-supply-chain trigger reads narrowly —
   2026-08-05.** Live, and its re-open condition (the flip to `stable` at
   `v1.0.0`) is unmet. **Verified to have no other home** — a grep across
   `RELEASING.md`, `lifecycle-kit/SPEC.md` and `docs/install.md` finds the
   narrowing recorded nowhere else — so §Where the grounds live keeps it here
   rather than pointing. It is neither spent nor stale, so it is **not
   shortened**: compressing a live ruling is outside both triggers.

*Keep, corrected in place:*

8. **The payload ships a prebuilt gate binary, selected by platform.** The
   ruling is live; its closing sentence is stale. It names
   `native-artifact-publish-path` and `native-artifact-install-path` in the
   present tense as the units that produce and place the artifacts, and both are
   done (verified: absent from the live slug set). The sentence drops the two
   slug names and keeps the pointers to gate-sdk/SPEC.md §Porting a gate to the
   binary substrate and §Consumer payload, which own the shipped mechanism.
9. **`install-profile-seam` cuts three units — 2026-08-08.** This iteration's
   own governing record, so it is live and **may not vanish** — and it is stale
   in three places, each corrected in place:
   - *cuts three units* → **four**. The operator widened the cut with this unit;
     a fourth unit present is the operator's own widening, not an envelope
     breach, and the next session to read the cut must find that.
   - *Whether [unit 3] emerges as debt or as a feature is the design's to
     decide* → settled: `lock-own-file-narrowed-profile-drift` was **ruled debt
     at spec**, and the ruling and its grounds live in that queue entry.
   - *if `spec` finds units 1 and 2 cannot honestly share an iteration, that is a
     finding to escalate* → spec ran and escalated no such finding; the clause
     goes.
   The correction also **states the ruling's own discharge event** — this
   iteration's close — per D1's authoring convention, so the close that ends
   `install-profile-seam` deletes it without re-deriving the judgment. That is
   the contract demonstrating itself on its first instance.

*Delete — spent. Each ground is a fact about the tree, verified, not an
argument:*

- **The first tag that publishes binaries is the operator's call — 2026-08-04.**
  Self-declared spending clause (*spent once that tag is cut*); `v0.22.0` cut it.
- **That tag is cut at `native-first-port-cohort`'s close — 2026-08-06**, and
  **The consent carries, and the next close cuts the tag — 2026-08-07.** Both
  subordinate to the above; both subjects finished.
- **The 2026-08-08 CORRECTION** appended to the last of those. Its parent is
  deleted, and its own content — which tag is newest, which Release carries a
  binary — is ordinary tree state a session reads from `git tag`, not a ruling.
- **The two-line `ruling-record-condition-staleness-probe` observation** that
  follows it. Not a directive; it is a design input to that entry, and D8
  relocates it there.
- **`init-claim-stickiness` enters through the next scope's standing directive —
  2026-08-04.** Self-declared: *Discharged 2026-08-05… a later scope owes it
  nothing.*
- **The next iteration's subject is delegation burn — 2026-08-06**, with its
  two follow-on paragraphs, and **The cut is five units — 2026-08-06.**
  `delegation-burn-reduction` is closed; its five units are done.
- **`native-cohort-activation` cuts four units — 2026-08-07**, with its honest
  limit. That iteration is closed and all four units are done.
- **The 2026-08-07 CORRECTION** appended to it. Its parent is spent, and its one
  live finding — that a port checks two questions, *which trees declare a
  descriptor* and *which trees register it* — is **already owned verbatim** by
  gate-sdk/SPEC.md §Porting a gate to the binary substrate, so nothing relocates.
  Its closing paragraph is the append-never-strike rationale D1 retires; the
  retirement is what licenses deleting the block that states it.
- **`adopter-floor-integrity` cuts three units — 2026-08-08**, with its honest
  limit. That iteration closed the same day this unit was ruled; all three units
  are done, and the honest limit's caution was addressed to the `spec` that has
  since run.
- **Pruning-on-completion is confirmed for this file — 2026-08-08**, with its
  four follow-on paragraphs. **Discharged by this unit**: its live content is
  D1, its explicit non-settlements are what the operator's directive supplies,
  and its honest limit (*until that trigger and authority exist, this ruling
  buys nothing mechanically*) is answered by D1 and D5. The unit eating its own
  ruling is the correct outcome, not an oversight to flag.
- **The published-Release backfill is executed by the build session —
  2026-08-08**, with its three follow-on paragraphs.
  `published-release-channel-flag-unheld` is done, so the authorization is
  spent; and the consent half — that no Release is Latest while the line is
  `0.x` — is **verified already owned** by `docs/install.md` §The release
  channel, which states it as the honest presentation of a preview channel. The
  paragraph itself names that section as the mechanism's home once the amendment
  merged, and it has.

**Expected shape after the pass, as an expectation build measures rather than a
number to hit:** §The closed rulings falls from 461 lines to roughly 125, and
the file from 593 to roughly 250. **No live ruling is shortened to reach it.**

**D3 — `§PRIORITY DIRECTIVE`'s discharged steps are deleted, and the standing
rules attached to them survive the deletion.** `{design-bearing}`

The section is kept — it is a directive in the operator's sense — and the
spent-⇒-delete trigger applies inside it like anywhere else. Steps 1–4 are
discharged against tree evidence (the vendoring model, the publish pipeline, the
first ported cohort, and `installer-lifecycle-verbs`; the scope survey's witness
was re-run at this session and holds — corpus unchanged on every load-bearing
path, oracle verdict unchanged). A discharged step directs nothing, so the six-
step list becomes the two that remain: `prose-profile` completion, then
`companion-toolkit-profile`. Both verified live.

**Three things inside the section are live and must survive the renumbering** —
this is why the delta is design-bearing rather than a list edit:

1. **The measurement point.** *Per-profile coherence, not whole-corpus
   completion*, re-ruled 2026-08-06 to fix where that coherence is measured — at
   the adopter's floor, install / get value / uninstall, and never at a roster
   substrate census. This governs the whole track, not the deleted step 3, and
   other rulings cite it.
2. **Criterion 4's re-entry condition.** The refusal to relax it was scoped *for
   the first cohort only*, and the first cohort is done — so the refusal is spent
   and the **named re-entry condition is now the live rule**: the
   criterion-clearing corpus exhausted *and* the parity oracle held off the shell
   substrate. Deleting the spent refusal while dropping its condition would
   silently widen what a second cohort may do.
3. **The closing paragraph** — `instruction-surface-bash-focus` unblocking on a
   threshold, and the surge-channel gating. Both live and untouched.

**D4 — `§The objectives` is a checked no-op.** `{mechanical}`
All six objectives and the *What the objectives are not* clause were read
against the tree at this session and none is spent or stale. Recorded as a delta
so build finds the no-op ruled rather than hunting for a correction to make.

**D5 — `canon-kit` gains `CANON_KIT_TEMPORAL_MARKERS_EXTRA`.** `{design-bearing}`
The enforcement rides an existing gate rather than a new one, because
`check-manifest-temporal`'s invariant is *verbatim* this ruling's ground: *a
manifest states current behavior only — history is derivable from git*. It
already scans `TRAJECTORY.md` (this repo's `CANON_KIT_MANIFEST_FILES` names it).
What it lacks is a way for a consumer to extend the marker set without replacing
it.

`canon-kit/lib/spec.sh` gains a third `_EXTRA` merge beside the two that already
exist for `CANON_KIT_PROSE_TELL_PHRASES` and `CANON_KIT_PROSE_TELL_ABBR_ALLOW` —
default empty, unioned onto the base after the base defaults resolve. The
semantics, including the replace-to-narrow valve and the provenance-seam ground,
are already stated generically in canon-kit/SPEC.md §Layout and configuration;
that prose covers a third member with no new rule, and the knob table gains the
name.

**D6 — `scripts/canon-config.sh` names this project's ruling-record idiom.**
`{mechanical}`
Consumer config, for the same provenance-seam reason the file's install-transport
and payload-disclosure vocabularies are: a spelling of one project's
ruling-record dialect is not kit mechanism.

The set is `correction appended`, `stands unstruck`, `when written`, and
`the ruling is spent`. **Measured across all 88 governed manifest files at this
session: every hit is in `TRAJECTORY.md`, and every hit is inside content D2
deletes.** Zero hits anywhere else, so the marker set costs no rewording
elsewhere in the tree.

The first three catch the appended-correction idiom D1 retires. The fourth is
the stronger one: it catches a ruling *labelled* finished and kept as a fixture,
which is the exact failure this unit exists to end.

**`is spent` was weighed as the fourth marker and narrowed.** It matches four
sites rather than two, all in `TRAJECTORY.md` and all inside deleted content —
so it is measurably safe today. It is narrowed anyway because D1's contract
paragraph must be statable without tripping a marker, and a marker that
constrains how the rule itself may be written is a marker that will be valved.

**The honest limit, and it is not small.** These markers hold the *idiom*, not
the *act*. A spent ruling written in different words passes, and a session that
learns the markers can route around them. The gate raises the floor and does not
close the class — what closes it is D1's authoring contract plus
`ruling-record-condition-staleness-probe`, and D9 records why nothing stronger
is buildable inside this unit.

**D7 — `CLAUDE.md`'s `TRAJECTORY.md` bullet drops the re-verify duty.**
`{design-bearing}`
It reads today: *a recorded ruling is closed, so re-verify facts against the
tree and escalate rather than reverse one.* The re-verify half exists because
the file is known-stale, and correcting at the source is what retires it —
keeping it would tax every reader for a staleness that no longer exists. The
escalate-rather-than-reverse half stays, and the bullet gains the retirement
distinction in one clause: retiring a spent ruling is not reversing one. The
one-line-per-rule always-loaded shape is preserved; the mechanism stays behind
the pointer.

**D8 — `ruling-record-condition-staleness-probe`'s stale premise is corrected,
and it absorbs the relocated observation. Landed at spec, in the promotion
commit.** `{mechanical}`
The entry survives as its own deliverable — its separateness is a recorded
operator preference, and *detect* and *act* stay complementary. Its stated
premise had moved: *Retiring a recorded ruling is operator-class, so a stage
session may not edit one. The enforceable half is a probe, never a prune.* The
operator has authorized the prune, so that sentence now records that the
authority ground moved while the probe/prune boundary holds on the deliverable
instead; what survives is the entry's real scope, a probe over
**condition-bearing** rulings plus the declaration that makes a prose condition
machine-readable. It also absorbs the two-line observation D2 removes from
`TRAJECTORY.md` — that the surface written to record a condition-miss went stale
itself inside a day, so the class the probe is filed against reaches corrections
and not only conditions.

**This delta is executed rather than pending**, because it is a queue write on
`spec`'s own surface and a stale premise left standing until build is a premise
a reader can act on. Build's obligation is to verify it, not to re-apply it —
recorded here so a build session finds it done rather than absent.

**D9 — two enforcement routes are refused with their grounds, so build does not
re-derive them.** `{design-bearing}`

- **No prune-happened stamp.** The mechanized-close-step shape
  (`check-lesson-disposition`'s evidence file) is available and is refused on the
  file's own header: *nothing mechanizes "the operator closed this", and nothing
  should — the alternative is a session attesting to its own consent, which is
  worth less than the rule.* A stamp saying a session pruned is a session
  attesting to its own diligence. Same class, same verdict.
- **No slug-liveness assertion over the record.** `check-queue-slug-liveness`
  exists and resolves bold-code slugs on configured prose surfaces, and adding
  `TRAJECTORY.md` to `QUEUE_KIT_PROSE_SURFACE_GLOBS` was probed: **it is a
  no-op** — the file carries 37 slug mentions and zero in the bold-code
  membership form, because it cites rather than claims membership. That is
  queue-kit's design, not an oversight: it rules an unresolved citation *not an
  error* precisely because prose legitimately names landed work. Loosening the
  grammar would be wrong for the same reason. Measured: 26 of those 37 names are
  not live, and the ruling this unit **keeps** (the 2026-08-05 release
  narrowing) cites two closed iterations as its *evidence*. Prose cannot
  distinguish a ruling's subject from a ruling's evidence, so a sound assertion
  needs a declaration — and that declaration is D8's entry, which this unit must
  not absorb.

## Producers and consumers

**`CANON_KIT_TEMPORAL_MARKERS_EXTRA`** — Producer: `scripts/canon-config.sh`,
this repo's canon-kit consumer config, sourced by `canon-kit/lib/spec.sh` on
every gate run; the merge happens in the lib after the base defaults resolve, so
the effective set is base plus extra. Enabling config: the assignment itself, on
the real gate path — the pre-commit hook and `run-gates.sh` both source the lib,
so the producer is reachable in the tree that matters and not only under
fixture. Consumer: `canon-kit/checks/check-manifest-temporal.sh`, which reads
`CANON_KIT_TEMPORAL_MARKERS` as one array at scan time and needs no change to
consume the extension — the union happens before it runs. Named reader for the
new knob: `canon-kit/lib/spec.sh`'s merge, at the transition where consumer
config has been sourced and gate dispatch has not yet begun. No other component
reads it.

**The retirement authority (D1)** — Producer: the operator's directive,
recorded in `TRAJECTORY.md`'s header where the file's other three authoring
rules live. Consumers, three and all live: any session meeting a spent ruling in
the record; `CLAUDE.md`'s `TRAJECTORY.md` bullet (D7), which is the always-loaded
pointer every session reads before it reaches the file; and this iteration's own
close, which D2 item 9 hands an explicit discharge event. **This is an authoring
contract with a named limit, not a mechanism** — D5 holds two idioms and D9
states why nothing holds the act.

**No new state, message, field, file, or directory is introduced.** The one new
name on a governed surface is the knob, and it has the named reader above. That
is also what makes this unit a feature rather than debt under the new-names
litmus.

**`check-manifest-temporal`'s existing verdict is the falsifier for D2 and D6
jointly.** After the pass it must be green with the four markers armed. A red
means either the prune missed a block or a marker is wider than measured, and
both are findings to resolve that session rather than valve — a
`manifest-temporal-exempt:` comment landed on a ruling this unit was supposed to
delete would be the defect wearing the gate's own valve.

## Existing sections updated

- **`TRAJECTORY.md` §How to read a ruling recorded here** — D1: retirement is not
  reversal; the re-verify duty drops.
- **`TRAJECTORY.md` §Who may record a ruling here** — D1: the symmetric
  retirement authority.
- **`TRAJECTORY.md` §Where the grounds live** — D1: the completion-time half,
  its two triggers, and the discharge-event authoring convention.
- **`TRAJECTORY.md` §The closed rulings** — D2.
- **`TRAJECTORY.md` §PRIORITY DIRECTIVE** — D3.
- **`TRAJECTORY.md` §The objectives** — D4, a ruled no-op.
- **`canon-kit/SPEC.md` §Layout and configuration** — D5: the knob table gains
  `CANON_KIT_TEMPORAL_MARKERS_EXTRA`, and the `_EXTRA` paragraph that already
  states the union semantics and the narrowing valve generalizes to name a third
  member rather than repeating the rule.
- **`canon-kit/SPEC.md` §check-manifest-temporal** — D5: the *Markers are
  `CANON_KIT_TEMPORAL_MARKERS`* sentence states that the set is base plus the
  consumer's extension.
- **`CLAUDE.md` §Housekeeping** — D7, the `TRAJECTORY.md` bullet.
- **`TASK-QUEUE.md`, `ruling-record-condition-staleness-probe`** — D8.

## Definition of Done

- [ ] **Causal completeness** — every new state/event/interface has a named,
      reachable producer and a named consumer; every new field has a named
      reader at a named transition.
- [ ] **Merged with no information lost** — each addition integrated into its
      proper canonical-spec section (not appended); the merged spec reads as one
      coherent document a reader who never saw the amendment can use alone.
- [ ] **Amendment deleted** — this file removed on merge; none remain for the
      component (`ls SPEC-*.md`).
- [ ] **Removals propagated** — grepped every spec for names this change
      retired; nothing dangles. Specifically: no surface still tells a session to
      re-verify `TRAJECTORY.md` against the tree, no surface still states the
      append-never-strike precedent, and no deleted ruling is cited by a
      surviving one.
- [ ] **`check-manifest-temporal` green with the four markers armed**, and the
      marker set verified to have zero hits outside `TRAJECTORY.md` across the
      governed manifest — the joint falsifier for D2 and D6, discharged by
      running the gate rather than by reading the diff.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed as
      debt tasks (a build-time causal gap is resolved that session, not
      deferred).
