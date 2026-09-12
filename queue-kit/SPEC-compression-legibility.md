# SPEC amendment: compression-legibility

Queue entry: `entry-compression-contract-unenforced`. **Operator direction,
2026-09-12**, promoting this entry into unit set `registry-roster-oracles` the same
day as, and against, a decline standing above it on the same entry.

## The ruling this amendment is authored against, and how it is held

**The 2026-09-09 consult ruling is not reversed and stands.** It ruled the entry
*costed and filed, no mechanism owed*, on this ground: the candidate arm's shape —
**a deferred entry's counted extent shrinks in a commit that also lands a ruling
line** — is *also* the shape of the relief §check-queue-entry-budget mandates
(compress by answering), so such an arm reds the correct act as often as the
defect. A classifier no gate can honestly run.

**That ground is untouched here, and this amendment proposes no classifier.**
Nothing in it reds. The refused arm's defect is that it issues a **verdict** over
an act whose two classes are semantically indistinguishable from the artifact; the
ruling is therefore a ruling about *reding*, and it is load-bearing for exactly
that shape. Reversing it still takes a `/consult`, and nothing below asks for one.

What this amendment builds instead follows from reading the entry's own statement
of the harm, which is **not** a classification failure:

> "extent is the only artifact, and a compressed entry reads identically whether
> its missing grounds were answered or silently discarded. **A later reader cannot
> tell that grounds it lacks were ever written.**"

That is a **legibility** failure. The attested instance is the same shape: at
`40477bdb` a compression displaced a propose-once record, and the next scope had
to recover the spent route from git to avoid re-escalating an answered member. The
information was never destroyed — it was in git the whole time, and nothing
pointed the reader at it. A reader who can ask *"was anything ever written here
that I now lack?"* and get a git-derived answer has the loss closed without anyone
classifying anything.

So: **one stated rule at the authoring tier, and one advisory oracle that judges
nothing.** The advisory tier is not invented for this unit — it is the tier
guard-kit's friction log already occupies, "advisory — triage at close, not a
gate" (guard-kit/SPEC.md:2420-2425).

## What changes

### (1) A compression's answer is durable on a governed surface before the compressing commit

§check-queue-entry-budget's compression contract gains its missing half: the rule
already says compress by *answering*, and now says where the answer has to be when
the ground leaves {design-bearing}.

The section today states the contract and concedes the gate cannot hold it: "it
sees an entry's current extent, and judging whether a removed line was answered or
discarded is semantic" (`queue-kit/SPEC.md:1711-1713`). What it does not state is
what *answering* obliges. Answering a ground means the ground is resolved — and a
resolution the compressing commit does not leave anywhere is a resolution the next
reader cannot use, which is indistinguishable from a discard by exactly the
measure the entry names.

**The rule already exists one component away, for the adjacent act, and this delta
is the missing sibling rather than a new obligation.** canon-kit states it for the
*landing* commit, as a consequence of its bidirectional rule: "prose that is worth
more than the entry's own lifetime belongs in a governed surface *before* the
landing commit, never on the entry as its permanent home"
(`canon-kit/SPEC.md:926-934`). An entry leaving the queue and an entry shedding
half its body are the same information event at different scales, and only the
first of them carries the rule. The delta states it for compression, cites
canon-kit as its origin rather than restating its reasoning, and inherits the
honest limit canon-kit already records: **no gate can catch it**, because the loss
is indistinguishable from an ordinary edit.

**The lever is the one the consult ruling itself named**, and the delta says so
where the ruling's reader will look: content tiering — a ruling lands on the entry
as a pointer to `TRAJECTORY.md` or the owning SPEC, never as prose. So the rule's
routine discharge costs nothing extra: the answer was going to a governed surface
anyway, and what the delta adds is that the pointer's target exists **first**.

### (2) `--emit entry-history <slug>`: the commits at which an entry's counted extent fell

A new advisory arm reports one line per commit in which the named entry's
**counted** extent decreased, with the count before and after and that commit's
subject {design-bearing}.

**It issues no verdict and it cannot red.** It reports commits. Whether a given
decrease was a legitimate compression, a relocation or a discard is left to the
reader, who has the commit in hand and can read it — which is the judgment the
consult ruling correctly says no gate can make, made by the party that can make it.
This is the whole of why the shape clears the ruled ground: the refused arm's harm
was a wrong red, and an arm with no red condition has no wrong red to issue.

**It mints no second spelling of the count.** The extent is the range
§check-queue-entry-budget assertion A already measures and the `queue-index` arm
already yields with `--extent`; the count is that extent less the declaration
discount the assertion already applies. The arm calls that computation, so a
change to what counts moves both at once.

**The walk is bounded and the bound is stated.** Commits are walked newest-first
over the queue file and the walk stops at the first commit in which the slug is
absent — the entry's own filing commit — so the cost scales with an entry's age,
not the repository's. The blobs stream through one batch read rather than a process
per commit. Measured at this authoring: 1999 commits touch the queue file in all,
464 of them since the subject entry was filed.

**Two honest limits, stated rather than discovered.** A decrease is attributed to a
slug, so an entry **renamed** mid-history reads as filed at its rename. And a
commit that compresses one part of an entry while growing another nets out and does
not appear — the arm reports the counted total, which is the quantity the cap binds
and therefore the quantity a displacement is measured against.

### (3) Assertion A's failure text names the arm beside the reliefs it already routes to

The one reader with a real trigger is the session the cap just blocked
{design-bearing}.

Assertion A's failure already routes a blocked session into
§check-queue-entry-budget — the section records this as the path its consumers take
(`queue-kit/SPEC.md:1698-1699`). That session is, by construction, the session
about to compress. It gains the arm's invocation beside the reliefs the text
already names, because what it most needs to know before compressing is **what has
already been compressed out of this entry** — so it does not re-answer a ground
answered two iterations ago, or drop one that was never answered at all.

This is what keeps delta 2 from being an arm nobody runs. The producer is a
failure a session is already reading; the consumer is that session at that moment.
No new trigger, no new schedule, and no new invocation point.

### (4) The arm declares itself advisory, and no collision counter is minted

The section records the arm's tier and re-states its existing refusal of a
collision counter as unaffected {mechanical}.

§check-queue-entry-budget already refuses a collision counter, on the ground that
it would be a second measurement of a quantity the headroom line prints, against a
criterion that reads composition rather than collision history
(`queue-kit/SPEC.md:1650-1656`). **The arm is not that counter and the section says
so**: it reports *which commits* reduced one named entry, on demand, and counts
nothing across the pool. Recorded because a reader who knows the refusal will
reach for it here, and because an arm that later grew a pool-wide tally would be
re-minting exactly what that paragraph refused.

## Producers and consumers

- **The durability rule** (delta 1).
  - *Producer:* the sessions §check-queue-entry-budget already binds — any session
    compressing a deferred entry to fit a mandated write. No new trigger, no new
    field, nothing to configure.
  - *Consumer:* that same session, reading the section when assertion A blocks it —
    the path the gate's failure text already routes it down and which delta 3
    widens by one line.
  - *Honest limit, inherited and named:* no gate reads this rule, exactly as no gate
    reads the compression contract it completes or canon-kit's sibling. It is an
    authoring contract, and delta 2 is what makes a break in it *detectable after
    the fact* rather than enforced before it.
- **`--emit entry-history <slug>`** (delta 2).
  - *Producer:* the crate's emit dispatch, over the git history of the queue file
    and the extent computation assertion A already performs.
  - *Consumer:* a session blocked by assertion A (delta 3), and discretionarily the
    scope drain that ranks the deferred pool — the reader the attested instance had.
  - *Every emitted field has a named reader:* the commit hash is read to open the
    commit, the before/after counts are read to size what left, and the subject is
    read to decide whether opening it is worth it. There is no fourth field because
    there is no fourth reader — in particular the arm emits **no verdict column**,
    and its absence is the delta's central property rather than an omission.
  - *Red condition:* the arm has none. It exits non-zero only on a usage error — an
    absent slug, or a slug no commit in range carries — which is the harness-error
    class, never a finding.
- **The failure-text line** (delta 3).
  - *Producer:* `check-queue-entry-budget`'s assertion A failure path.
  - *Consumer:* the blocked session, at the moment the cap blocks it.
- **Red conditions of the affected readers under a narrowing** (causal-completeness
  point 5). **No delta narrows a corpus.** Delta 1 is prose in a SPEC section;
  delta 2 adds an arm; deltas 3 and 4 add text to an existing failure path and an
  existing section. The readers that could nonetheless flip:
  - `check-queue-entry-budget` itself reds on assertion A (a count over the cap),
    B (an icebox continuation line), C (an absent cost lead-in) or D (a `ruled:`
    line). **No assertion changes**, and the count's definition is untouched — delta
    2 *reads* the computation rather than altering it, which is why it cannot move
    a verdict.
  - `check-queue-wrap` reds on an over-width line. Delta 3 adds a line to a gate's
    failure **text**, not to a queue entry, so it is outside that gate's corpus.
  - `check-spec-pointer` and `check-md-refs` red on an unresolvable pointer or
    link. Delta 1 cites `canon-kit/SPEC.md` by section, so the citation must
    resolve at the landing commit — named because it is the one reader this
    cross-component citation puts at risk.
  - `check-crate-arms` reds on a failing crate lint or test arm; the unit tests land
    with the arm.
  - `check-gate-fixture-coverage` reds on a registered gate with no fixture pair.
    Delta 2 is a non-gate arm, not a gate, so it is outside that gate's derived set —
    it takes a bespoke test under `gate-tests/` on the pattern §Layout and
    configuration already rosters for guard-kit's arms.

## Existing sections updated

- `queue-kit/SPEC.md` §check-queue-entry-budget, the compression paragraph at
  1708-1714 (delta 1). It gains the durability clause and its citation of
  canon-kit's sibling rule; the existing concession that the gate cannot hold the
  contract stays exactly as written, because the delta does not make it holdable.
  **Not yet applied.**
- `queue-kit/SPEC.md` §check-queue-entry-budget (deltas 2, 3 and 4). A new passage
  owns the arm: its report, its bounded walk, its two limits, its advisory tier,
  and the statement that it is **not** the collision counter the section refuses
  eighty lines above.
- `queue-kit/SPEC.md` §check-queue-entry-budget, the collision-counter refusal at
  1650-1656 (delta 4). One clause pointing forward to the arm and recording that
  the refusal is unaffected, so the two passages are not read as contradicting each
  other. **Not yet applied.**
- `gate-sdk/SPEC.md` §The non-gate arm (delta 2). `--emit-entry-history` joins the
  roster with its owning section, as every arm there does.
- `queue-kit/SPEC.md` §Layout and configuration (delta 2). The arm's bespoke test
  joins the `gate-tests/` roster.
- `docs/queue-kit/SPEC.md` and `docs/gate-sdk/SPEC.md` (all deltas) — the generated
  on-site mirror; regenerate with
  `bash gate-sdk/bin/run-gates.sh --emit docs-mirror --write`.

**Deliberate non-updates, recorded so a reader does not go looking.**
`TRAJECTORY.md` is unchanged: the consult ruling stands, is not discharged by this
amendment, and an entry that carried its own discharge would be reversing it by
side effect. `canon-kit/SPEC.md` is unchanged: delta 1 cites its arm-(c)
consequence as the origin of the rule rather than moving or duplicating it, which
is the content-tiering direction — the sibling that exists stays the owner of its
own reasoning.

## Retired spellings

- None — no delta removes a name. `extent` and `count` keep their definitions and
  their at-most-one-per-grammar discount; `compression by answering` keeps its
  spelling and gains a clause; the collision counter stays refused under the name
  it was refused as.

## Definition of Done

- [ ] **Causal completeness** — every new state and interface has a named,
      reachable producer and a named consumer; every new field has a named reader
      at a named transition.
- [ ] **The standing ruling is intact at the landing commit.** The 2026-09-09
      consult ruling is not annotated, not re-verified, not marked discharged, and
      not cited as superseded. Its ground — that a classifier over the compression
      act cannot honestly red — is true of the tree after this unit exactly as
      before it, because this unit ships no such classifier.
- [ ] **Nothing reds.** Verified rather than asserted: the arm has no exit-1 path,
      and no gate's assertion set grows.
- [ ] **One spelling of the count** — the arm calls assertion A's own extent and
      count computation; a test asserts the arm's reported count for an entry equals
      the headroom line's for the same entry at the same commit.
- [ ] **The walk's bound is exercised** — a test over a seeded history confirms the
      walk stops at the filing commit rather than at the root, and the two stated
      limits (a renamed slug, a net-zero commit) each have a case recording the
      behaviour rather than a claim about it.
- [ ] **The blocked session actually meets the arm** — assertion A's failure text is
      read from a real firing, not from the diff.
- [ ] **This edit is its own subject, and pays for itself the way it mandates.** The
      queue entry is at the cap; any line this unit lands on it is funded by
      compressing answered grounds with the relocation named, never by dropping an
      unanswered one — and any answer displaced lands on its governed surface in the
      same commit, which is delta 1 applied to the commit that writes delta 1.
- [ ] **Full battery green** (`bash gate-sdk/bin/run-gates.sh`),
      `bash gate-sdk/bin/build-native.sh`, and the queue-kit and gate-sdk fixture
      suites.
- [ ] **Merged with no information lost** — each addition integrated into its proper
      canonical-spec section, not appended.
- [ ] **Amendment deleted** — this file removed on merge; none remain for the
      component (`ls queue-kit/SPEC-*.md`).
- [ ] **Removals propagated** — `## Retired spellings` above holds, re-run by
      `check-amendment-retired-spelling`.
- [ ] **Gaps filed** — a cross-component gap found during the work is resolved that
      session, not deferred.
