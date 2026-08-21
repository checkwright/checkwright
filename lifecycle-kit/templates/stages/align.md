The `align` (spec-alignment) stage of an iteration. Cross-spec audit, then
*<consistency-gate: your aggregate consistency gate — e.g. the full gate
battery>* as one gate. Exit condition: no unresolved conflicts or terminology
drift.

**First step — stamp evidence.** Run lifecycle-kit's `bin/enter-stage.sh
align`: it appends `<iteration> align <session-id> <date> <head>` to
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
explicitly rule it unwarranted and a `<iter> align-waived <session> <date> <head>`
line is recorded in `.workflow/WORKFLOW-STATE.txt` — never self-issued by the
entering build session (lifecycle-kit/SPEC.md §check-stage-entry).

## Session ritual

*<audit-fanout: your audit fan-out: sweep the spec corpus for missing fields,
naming inconsistencies, undocumented contracts, contradictions; name the
entry grammar any backfill task filed from a finding is written against.>* Resolve every
finding in the affected spec or amendment — never ad-hoc, never deferred.
When both could hold it, **the surviving surface wins**: an amendment is deleted
at merge, so a correction written into one and deleted in the same commit that
lands it leaves no trace in history at all — the diff shows a file added and
removed, and the reasoning is unrecoverable by any later reader. Land it in the
canonical spec the amendment merges into, and let the amendment cite it.
Commit all spec changes in a single `chore:` commit.

**A sweep is a survey — check the record before you buy one, and file the one
you buy.** The audit fan-out above is the most expensive survey any stage
dispatches. Before dispatching it, read the survey record and run the witness on
any block whose heading already answers your question; afterwards, file the
finding a later stage will want. Both halves, and what a passing witness
licenses you to cite, are lifecycle-kit/SPEC.md §The survey record.

**Audit the amendment against itself before auditing it against the tree.** Two
defects are visible on the amendment alone and both survive a green battery. An
author-stated count ("three things and no fourth") is an assertion about the
deliverable, so check it against what the deltas actually mandate rather than
against the sentence it heads. And every `## Existing sections updated` entry
must name the delta that owns it (canon-kit/templates/SPEC-amendment.md).

**A grammar the amendment states about a tool's behaviour is run against that
tool.** An amendment modelling something outside itself — a filter language, an
option parser, a file format — is a *claim about a program*, and reading it is
not testing it. One invocation settles it. Attested: an amendment specified a
pin-path grammar admitting a leading bracket step, the gate refused any path not
opening with `.`, and the tool being modelled read a leading `["k"]` as an array
literal rather than an index — a three-way disagreement that passed authoring and
this audit and was caught by the first differential run at build.

**The `## Existing sections updated` roster is checked from the tree, not from
the amendment.** A roster entry with no delta is visible on the document; a
*surface with no entry* is not, because the evidence is in the tree. When a delta
replaces a literal — a version, a floor, a policy phrase — grep the tree for the
old one and reconcile every survivor against the roster. Attested twice in one
iteration, both times a documentation surface reading as commentary that the
grep found and the amendment did not name (`amendment-roster-omission-detection`
owns whether any of this is gateable).

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
