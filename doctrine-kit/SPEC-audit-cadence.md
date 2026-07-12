# SPEC amendment: audit-cadence

## What changes

**Placement ruling: extend Enforcement-first's carve-out clause, not a new
numbered rule** — the clause is the parent ("the class a check cannot decide
cleanly stays a stated manual duty rather than a noisy gate"), and a new rule
would force a second renumber beside the gap-disposition one. The clause
gains the cadence duty: a stated manual duty carries a *named cadence* — the
un-gateable class joins a tracked audit roster reviewed on a lifecycle hook,
because a duty with no cadence is a duty no session performs; it rots exactly
like the deferred gap the Gap-disposition rule catches.

**Cadence-home ruling.** The review rides the close stage — close already
walks the iteration's residue, and a per-iteration review is the tightest
cadence the lifecycle offers without a new mechanism. The kit tier changes
nothing structural: the duty lives in the rule text (doctrine-kit), and each
consumer wires the review into its close ritual (this repo: the close shim's
ritual binding gains the roster-review step). No new kit knob, no new
template step — the doctrine owns the duty, the consumer's close binding is
the trigger.

**Roster ruling.** The roster is a tracked, hand-curated file — *not* a
derived artifact: which classes are un-gateable is a judgment no tool can
enumerate, so derivation-first's ladder lands on "state once at the owner".
This repo's roster is `.workflow/audit-roster.txt` (the committed `.workflow/`
projection home), one line per class:

    <class-slug> — <audit scope: what to sweep> — due: <event> — last: <iteration>

Due-ness is event-keyed, not counter-keyed — a named observable event (a
heavy-SPEC contract edit, a release prep, a template upgrade) beats an
iteration counter no surface tracks. The close review reads the roster,
judges which events fired since each `last:` stamp, performs or explicitly
defers the due audits, and updates the stamps.

**Seed member.** The internal-identifier-restatement class (the
spec-contract-leaning-sweep unit's residual — prose restating a source's
internal identifier roster, un-gateable because public names are legitimate
contract citations), due on any heavy-SPEC per-component-contract edit and at
release prep.

**Enforcement ruling.** Stated duty, no gate — audit cadence is not
machine-decidable; the carve-out is self-applying, and the roster file plus
the close step *are* the capture mechanism the extended clause demands.

## Producers and consumers

- **Producer:** any scope/build session that rules a class un-gateable (the
  Enforcement-first weighing) appends the roster line in the same unit.
- **Consumer:** the close-stage session — this repo's close shim binding gains
  the review step naming the roster path; the roster's `last:` field is
  written by that same step (its named reader is the next close's due-ness
  judgment).
- **Field readers:** `class-slug` (close review, dedup at append), `audit
  scope` (the session performing the audit), `due:` (close review's fired
  judgment), `last:` (close review's staleness judgment). No field without a
  reader.

## Existing sections updated

- `doctrine-kit/DOCTRINE.md` — Enforcement-first's carve-out sentence extended
  with the cadence duty and the roster pattern (generic form, no consumer
  path).
- `.claude/commands/close.md` (this repo's close shim) — ritual binding gains
  the roster-review step citing `.workflow/audit-roster.txt`.
- `.workflow/audit-roster.txt` — created with the header and the seed member.
- CLAUDE.md digest: the Enforcement-first bullet is regenerated via
  `install-doctrine.sh` only if its one-line digest wording changes; the
  extension aims to leave the digest line as-is.

## Definition of Done

- [ ] **Causal completeness** — every new state/event/interface has a named,
      reachable producer and a named consumer; every new field has a named
      reader at a named transition.
- [ ] **Merged with no information lost** — each addition integrated into its
      proper canonical-spec section; the merged doc reads alone.
- [ ] **Amendment deleted** — this file removed on merge; none remain for the
      component.
- [ ] **Removals propagated** — nothing retired by this change.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed
      as debt tasks.
