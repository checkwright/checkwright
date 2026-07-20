The `align` (spec-alignment) stage of an iteration. Cross-spec audit, then
*<consistency-gate: your aggregate consistency gate — e.g. the full gate
battery>* as one gate. Exit condition: no unresolved conflicts or terminology
drift.

**First step — stamp evidence.** Run lifecycle-kit's `bin/enter-stage.sh
align`: it appends `<iteration> align <session-id> <date>` to
`.workflow/WORKFLOW-STATE.txt` (required by `check-stage-evidence`; the stamp
proves invocation, not faithful execution), reading `<session-id>` from
`bin/session-id.sh`
(the newest transcript — never hand-picked), using `date +%F`, and refusing
(writing nothing) if `check-stage-entry` is red. On a refusal, **do not force
the entry** — escalate to the lead (where one exists and this is not a standalone
session) and stop; a refused entry is a gate verdict to resolve at its source,
never to override. That stamp *is* the
transition — the last stamp is the stage cursor, so nothing flips and no queue
write is involved. Commit the stamp on its own.

## Trigger (align is trigger-gated)

Run `align` only when one fires: (1) phase start, before the first
implementation task; (2) a multi-component spec ambiguity surfaces during
build; (3) this iteration's **authoring stage** (scope by default, or the
dedicated authoring stage where the roster splits one out) authored an amendment
changing ≥2 components' contracts. None firing → the prior stage advances
directly to `build`
(align skipped; the advance still needs user approval per the stage-line
rule). The arriving stage stamps its own entry as its first step
(above).

Trigger 3 is mechanized at build entry: `check-stage-entry` assertion C blocks
the build entry when the on-disk amendments carry a cross-component signal and
no `<iter> align` stamp exists. To skip the audit anyway, the user must
explicitly rule it unwarranted and a `<iter> align-waived <session> <date>`
line is recorded in `.workflow/WORKFLOW-STATE.txt` — never self-issued by the
entering build session (lifecycle-kit/SPEC.md §check-stage-entry).

## Session ritual

*<audit-fanout: your audit fan-out: sweep the spec corpus for missing fields,
naming inconsistencies, undocumented contracts, contradictions; name the
entry grammar any backfill task filed from a finding is written against.>* Resolve every
finding in the affected spec or amendment — never ad-hoc, never deferred.
Commit all spec changes in a single `chore:` commit.

**Every amendment's "wires cleanly against the current tree" is a hypothesis,
and the align audit is its first test.** Authoring a producer silently asserts
the consumer's read side already matches — it rarely does. Verify every
cross-component literal *at the read site*: grep the consumer's actual match
arms, don't trust the amendment's prose. A **negative** existence claim ("no
such harness/helper/gate exists yet") is the audit's weakest evidence shape: a
literal-string grep proves only that the spellings you guessed are absent, and
the thing you are about to build often already exists under a spelling you did
not anticipate. Before asserting absence, search the *concept* two ways — the
symbol under its plausible spellings **and** the directory that would own it —
or write the claim bounded ("no match for X") rather than absolute. When scope authored a new gate, the
audit is that gate's first real run — the drift it surfaces is a backfill
worklist to land before build, not a reason the gate is wrong.
