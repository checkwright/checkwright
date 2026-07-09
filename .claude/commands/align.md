The `align` (consistency-audit) stage of a Checkwright iteration —
trigger-gated; most iterations go scope → build directly. Exit
condition: no unresolved cross-surface drift.

**First step — stamp evidence.** Run
`bash lifecycle-kit/bin/enter-stage.sh align`: it appends `<iteration> align
<session-id> <date>` to `.workflow/WORKFLOW-STATE.txt` and flips the
TASK-QUEUE.md `[stage:]` line to `align`, committed together. It reads
`<session-id>` from `bin/session-id.sh` itself (never hand-picked), uses
`date +%F`, and refuses (writing nothing) if `check-stage-entry` is red.

## Trigger

Run only when one fires: (1) a kit lands that reshapes an existing kit's
contract (as lifecycle-kit reshaped gate-sdk's resolution); (2) a
cross-kit ambiguity surfaces during build; (3) scope ruled a change to ≥2
kits' contracts. Trigger 3 is mechanized by `check-stage-entry` assertion C
where amendments exist on disk; a waiver (`<iter> align-waived …`) is
recorded only on the user's explicit ruling.

## Session ritual

Audit the drifted surfaces against each other: each kit's SPEC.md vs its
code, README.md kit table vs reality, CLAUDE.md conventions vs the kits'
actual shape. Resolve every finding in the owning file; run the full battery
(`bash gate-sdk/bin/run-gates.sh`) as the aggregate gate. Single `chore:`
commit.
