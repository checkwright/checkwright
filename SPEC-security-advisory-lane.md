# SPEC amendment: security-advisory-lane

`.claude/commands/scope.md` §The GitHub boundary sweep opens with a closure
claim:

> `TASK-QUEUE.md` stays the sole owner of work state; nothing lives
> triaged-but-unqueued anywhere else.

and then enumerates the surfaces that claim is closed over: two lanes, Issues
and PRs. `SECURITY.md` directs reporters to a **third** GitHub intake surface —
private vulnerability reporting — and nothing on our side reads it. Advisories
are returned by neither `gh issue list` nor `gh pr list`, so the closure claim
is false for the one lane whose items are the most time-critical.

**Honest limit, restated so this amendment is not read as fixing a black
hole.** Advisories are not unread. GitHub notifies maintainers, so the lane is
*unswept*, not unwatched. What is missing is a systematic disposition duty, not
a notification.

**One inherited premise is dropped rather than carried.** The queue entry cites
`SPEC-supply-chain-trust-baseline.md` §causal chains as having named the route's
producer and mislabelled the external reporter its consumer. That amendment
merged and was deleted, and its A1 disclosure-route prose landed in **no**
canonical spec — the merge commit's own enumeration omits `SECURITY.md`, and the
text survives only in git history. There is therefore no live prose to correct,
and this amendment derives the chain fresh in §Producers and consumers rather
than inheriting a characterization from a deleted file.

## The diagnosis the entry did not have

The entry frames the lane as owing three decisions. The survey behind this
amendment found that **two of the three collapse into one, and the third
dissolves**, because the entry's framing attributes the problem to *severity*
when the thing that actually does not fit is *disclosure*.

- **Speed is not the gap.** The entry costs a sweep-lane duty as making
  acknowledgement latency equal to the time to the next scope entry. Measured
  against what `SECURITY.md` actually promises, that is comfortable rather than
  tight: the boundary cadence has run at roughly one iteration per day, against
  an advertised acknowledgement window of about a week and a first assessment
  of about two. A sweep lane meets the advertised windows with a wide margin.
- **The output axis is already carved out.** `SECURITY.md` §Supported versions
  already declares that "a security or supply-chain fix is a release trigger and
  does not wait out the ordinary cadence". The machinery that ships a fast fix
  exists and is exempt from the ordinary cadence today. Nothing about intake
  changes that.
- **What genuinely does not fit is that the lane's items are secret.** Every
  disposition surface the lifecycle owns — the queue, the committed gap inbox —
  is public. That is not a claim about *when* work may start; it is a claim
  about *where the record may live*. Treating it as a severity question produces
  an interrupt nobody needs; treating it as a disclosure question produces a
  disposition grammar, which is what the lane was actually missing.

## What changes

### Delta 1 — a third lane, in the consumer binding only *{design-bearing}*

`.claude/commands/scope.md` §The GitHub boundary sweep gains an **Advisories**
lane beside Issues and PRs, swept at the same point, under the same top-five
cap, so the section's closure claim becomes true.

The lane's probe is `gh api 'repos/{owner}/{repo}/security-advisories'`, taking
the repository advisories whose state is the reporter-submitted, awaiting-
maintainer one. The `{owner}/{repo}` placeholder resolves from the working
directory, matching the cwd-relative style of the two existing lanes (neither
passes `-R`). The existing sentence constraining these calls — that they run
only inside the interactive sweep and that no pre-commit or session-context hook
makes a network call — reaches the new lane unchanged and is not restated for
it.

**The closure claim is rewritten rather than deleted.** It stays a claim, and
the lane roster below it is what makes it true. Deleting it would remove the
sentence whose falsity is the only reason this defect was findable.

### Delta 2 — the disposition grammar, which is where the lane differs from its siblings *{design-bearing}*

Copying the Issues lane's dispositions would publish vulnerabilities. The Issues
disposition *promoted* means "a queue entry in this file's grammar, its body
citing the issue"; `TASK-QUEUE.md` is tracked and public, so a promoted entry
citing an unfixed vulnerability publishes it. The committed gap inbox is not an
escape either — `lifecycle-kit/SPEC.md` §The committed gap inbox makes it
committed *deliberately*, because a per-clone buffer fragments the backlog. Both
candidate carriers are public, and inventing a private one would resurrect the
gitignored buffer that section rules out.

So the lane's dispositions are stated over the advisory thread, and the rule
governing the public tree is stated as a prohibition rather than a format:

**Until the advisory is published, the advisory thread is the work record and
the public tree says nothing.** Not a redacted entry, not a placeholder slug,
not a count. A public entry naming a fix's shape against an unpublished advisory
is itself a disclosure vector — it tells a reader which surface to look at and
when to look — so partial disclosure is refused rather than formatted.

Four dispositions, each terminal:

- **Fix under embargo** — the work is tracked on the private advisory thread.
  The public tree gains its queue entry **at publication**, on the existing
  provenance sentence (`Surfaced <date> by security advisory <id>`), reusing the
  Issues lane's no-new-tag rule; `queue-kit/SPEC.md` §The tag algebra is
  untouched by this lane exactly as it is by the other two.
- **Advisory-only** — assessed, no code change owed. The advisory is published
  with the assessment; nothing enters the queue, and nothing is owed later.
- **Declined with cause** — the thread carries the reason and the advisory is
  closed. This is the Issues lane's *closed with cause* under embargo.
- **Not a vulnerability, but real work** — a report that is an ordinary defect
  leaves the lane and enters ordinary intake immediately, because nothing about
  it needs embargo. This disposition exists so the embargo rule cannot be used
  to park ordinary work out of sight.

There is no linked-and-skipped middle state, which is the Issues lane's gap-
disposition rule reaching this lane unchanged.

### Delta 3 — no severity carve-out, refused with its reasons recorded *{design-bearing}*

The entry's third question — whether scope-gated intake gains an exception so a
critical advisory preempts the running iteration. **Ruled: no.** Neither
`CLAUDE.md` §Delivery doctrine nor `doctrine-kit/DOCTRINE.md` item 11 is edited
by this unit, and the reasons are recorded here so the question is not
re-litigated as an oversight:

1. **The exception would buy latency nothing promises.** Per the diagnosis, the
   boundary cadence already meets the advertised windows with margin. An
   interrupt is machinery for a promise `SECURITY.md` does not make — and
   `SECURITY.md`'s windows are explicitly *not* tightened by this unit, which is
   what the entry's cost line demands.
2. **The fast half already exists** on the release axis, one level down from
   intake. Adding an intake interrupt would be a second, overlapping exemption
   for an outcome the first already delivers.
3. **The misfit is disclosure, not severity**, and Delta 2 addresses it where it
   lives. A rule about where a record may be written is not an exception to a
   rule about when work may start.
4. **A severity tier would be a new vocabulary the project deliberately lacks.**
   `SECURITY.md` carries no severity language at all; its only distinction is
   confirmed versus unconfirmed. Minting `critical` here would create a token
   whose only reader is a bypass — and "this is urgent" as a general bypass is
   precisely the over-action failure doctrine item 11 exists to catch. That item
   states it "licenses no self-exemption"; this unit takes it at its word.
5. **`doctrine-kit` is vendored.** A security carve-out in `DOCTRINE.md` ships
   one project's security posture as every consumer's delivery doctrine — the
   provenance seam, refused on that ground alone even if 1–4 had gone the other
   way.

The refusal is the deliverable of this delta. Recording it is not decoration:
the entry names the carve-out as the plausible-earning case, so an amendment
that merely omitted it would read as having missed it.

### Delta 4 — the sweep's prose sweep *{mechanical}*

Grep the tree for other statements of the two-lane closure — surfaces asserting
that the queue is the sole owner of triaged work, or enumerating the boundary
sweep's lanes — and correct what turns up. The ruling is fixed by Deltas 1–2;
the sweep executes it.

## Producers and consumers

This amendment adds a **duty and a disposition grammar** to a consumer binding.
It introduces no state file, no knob, no gate, no field. The causal pass is
therefore run on the lane itself, which is where a sweep duty can still go
wrong — by naming a probe nothing answers, or a disposition nobody performs.

**The advisory lane** (new duty, Delta 1).
*Producer, named and verified reachable:* GitHub's repository security
advisories on this repo, read via `gh api
'repos/{owner}/{repo}/security-advisories'`. This was **executed during
authoring**, not merely cited: the call returns an empty JSON array at exit 0,
and the same call filtered to the reporter-submitted state likewise. So the
producer is a live route with the placeholder expansion working from the
working directory — not an enabling configuration nobody sets. The upstream
producer of the *items* is an external reporter following `SECURITY.md`'s
route.
*Consumer:* the scope stage session performing the boundary sweep, at the same
point in the ritual that already consumes the Issues and PRs lanes. This is the
correction to the deleted amendment's chain, derived rather than inherited: the
reporter is the **sender**, so naming the reporter as the consumer left the
route with a producer and no reader, which is exactly the shape that let the
lane go unswept. The consumer is on our side of the boundary or the lane has
none.

**Each disposition** (new grammar, Delta 2).
*Producer:* the same scope session, one disposition per swept item.
*Consumers:* the advisory thread for all four (it carries the assessment, the
decline reason, or the embargoed work record); `TASK-QUEUE.md` additionally, for
*fix under embargo* at publication and for *not a vulnerability* immediately.
*Every field has a named reader:* the lane introduces exactly one new field —
the advisory identifier on the provenance sentence of a post-publication entry —
and its reader is a later scope or close session tracing that entry back to its
origin, the same transition at which `Surfaced <date> by GitHub issue #N` is
read today. No other field is added, and in particular no severity field is
added, because Delta 3 rules out the only reader it would have had.

**Enforcement, ruled explicitly.** This unit ships **no gate**. The predicate a
gate would need is *"does this repo advertise an intake route the sweep does not
read"*, and the set of routes is GitHub's, outside the tree — so a gate could
only compare two prose surfaces we author ourselves. Such a gate would have
passed the instant those two surfaces were written consistently, and it would
never have caught this miss, because the miss was that nobody thought of the
lane rather than that two surfaces disagreed. `gate-sdk/SPEC.md` §When a gate
earns its place refuses exactly that shape: a trivially-true proxy that
manufactures false confidence. The enforcement-first doctrine's own ordering is
satisfied by the other move it ranks higher — the false closure claim is made
true rather than gated.

## Existing sections updated

- **`.claude/commands/scope.md` §The GitHub boundary sweep** — the closure claim
  rewritten over three lanes (Delta 1), the Advisories lane and its probe added
  (Delta 1), its disposition grammar and the until-publication prohibition added
  (Delta 2). This is the amendment's whole surface.
- **`SECURITY.md`** — **not edited, deliberately.** Its windows are not
  tightened (Delta 3 reason 1) and its release-trigger sentence already carries
  the only security carve-out this project makes (the diagnosis). Listed here
  because a reader will expect the security policy to move and must see that the
  absence is ruled rather than overlooked.
- **`CLAUDE.md` §Delivery doctrine and `doctrine-kit/DOCTRINE.md` item 11** —
  **not edited**, per Delta 3. Listed for the same reason: the queue entry names
  the carve-out, so silence about these two surfaces would read as an omission.
- **No kit SPEC changes, no gate, no knob, no queue-tag change.**

## The seam

Ruled explicitly, per CLAUDE.md §The provenance seam.

**The lane is consumer rule content, and lands nowhere else.** The boundary
sweep lives in `.claude/commands/scope.md` — a *binding* filling the generic
`ritual` slot in `lifecycle-kit/templates/stages/scope.md`. The kit template
names no forge, no `gh`, no lane. Pushing an advisory lane down into
lifecycle-kit would ship this project's intake vocabulary — GitHub, advisories,
this disposition set — as every consumer's stage contract, and would bind
consumers whose forge has no such surface. **lifecycle-kit is not edited by this
unit.**

**Nothing is pulled the other way either.** No kit mechanism moves up into the
consumer; the lane composes from the `gh` calls the binding already makes.

**No knob is introduced, deliberately.** The tempting move is a
lifecycle-kit knob naming the consumer's advisory probe. It is refused on the
field-with-no-reader rule wearing config's clothes: nothing in any kit would
consume the value, because the lane's only reader is a stage session executing
its own binding. A knob added for symmetry is a governed name with no mechanism
behind it.

## Definition of Done

- [ ] **Causal completeness** — the lane's producer is named and was verified
      reachable by execution during authoring; its consumer is the scope session
      that already runs the sibling lanes; the one new field (the advisory id on
      a post-publication provenance sentence) has a named reader at a named
      transition. No severity field exists, by Delta 3.
- [ ] **Merged with no information lost** — the three-lane roster and the
      disposition grammar land inside §The GitHub boundary sweep in place of the
      two-lane text, not beside it; the section reads as one sweep with three
      lanes, and the until-publication prohibition is stated once.
- [ ] **Amendment deleted** — this file removed on merge; none remain at the
      repo root (`ls SPEC-*.md`).
- [ ] **Removals propagated** — Delta 4's sweep has run and no surface still
      states the two-lane closure; no severity vocabulary was introduced
      anywhere for a later reader to find dangling.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed via
      `bash lifecycle-kit/bin/file-gap.sh`. (The doctrine carve-out is settled in
      Delta 3, not filed; §Delta 3 records why nothing is owed.)
