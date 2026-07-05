The `align` (spec-alignment) stage of an iteration. Cross-spec audit, then
*<your aggregate consistency gate — e.g. the full gate battery>* as one gate.
Exit condition: no unresolved conflicts or terminology drift.

**First step — stamp evidence.** Append `<iteration> align <session-id> <date>`
to `.workflow/WORKFLOW-STATE.txt` (required by `check-stage-evidence`; the
stamp proves invocation, not faithful execution). As the same first step, flip
the queue header's `[stage:]` line to `align` and commit the flip together
with this stamp — the arriving-stage flip; the line and its stamp must match,
so they ride in one commit. Take `<session-id>` from lifecycle-kit's
`bin/session-id.sh` (it reads the id from the newest transcript — never
hand-pick it); `<date>` is `date +%F`.

## Trigger (align is trigger-gated)

Run `align` only when one fires: (1) phase start, before the first
implementation task; (2) a multi-component spec ambiguity surfaces during
build; (3) this iteration's `scope` authored an amendment changing ≥2
components' contracts. None firing → `scope` advances directly to `build`
(align skipped; the advance still needs user approval per the stage-line
rule). The arriving stage flips its own `[stage:]` line as its first step
(above).

Trigger 3 is mechanized at build entry: `check-stage-entry` assertion C blocks
the build flip when the on-disk amendments carry a cross-component signal and
no `<iter> align` stamp exists. To skip the audit anyway, the user must
explicitly rule it unwarranted and a `<iter> align-waived <session> <date>`
line is recorded in `.workflow/WORKFLOW-STATE.txt` — never self-issued by the
entering build session (lifecycle-kit/SPEC.md §check-stage-entry).

## Session ritual

*<Your audit fan-out: sweep the spec corpus for missing fields, naming
inconsistencies, undocumented contracts, contradictions.>* Resolve every
finding in the affected spec or amendment — never ad-hoc, never deferred.
Commit all spec changes in a single `chore:` commit.

**Every amendment's "wires cleanly against the current tree" is a hypothesis,
and the align audit is its first test.** Authoring a producer silently asserts
the consumer's read side already matches — it rarely does. Verify every
cross-component literal *at the read site*: grep the consumer's actual match
arms, don't trust the amendment's prose. When scope authored a new gate, the
audit is that gate's first real run — the drift it surfaces is a backfill
worklist to land before build, not a reason the gate is wrong.
