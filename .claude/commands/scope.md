The `scope` (design) stage of a Checkwright iteration (one iteration per kit,
in the extraction order README.md records). Read `BRIEF.local.md` (local-only
brief) and the platform sources for the kit being extracted; decide the kit's
layout, config surface, and de-hardcoding worklist; name the iteration after
the kit. Exit condition: the design is written down (this session's plan or
the kit SPEC draft) and the seam is ruled — what ships as mechanism, what
stays platform rule content, what becomes consumer config.

**First step — reset + stamp evidence.** Run
`bash lifecycle-kit/bin/enter-stage.sh scope`. `scope` is the iteration
boundary, so it truncates `.workflow/WORKFLOW-STATE.txt` back to its header
(git history keeps the prior iteration's stamps), stamps `— scope
<session-id> <date>`, and sets the TASK-QUEUE.md header to
`## Iteration: —  [stage: scope]` — the arriving-stage flip, here
bootstrapping an unnamed iteration. It reads `<session-id>` from
`bin/session-id.sh` itself (never hand-picked), uses `date +%F`, and refuses
(writing nothing) if `check-stage-entry` is red.

Scope rules for this repo: copy-first, never carve-out (the platform repo
stays untouched); rule content never leaves the platform — term lists,
vocabularies, glossary bodies become optional consumer config, never kit
literals; platform values stay as defaults under `<KIT>_<KNOB>` env/config
knobs.

**Triage every task at filing** (spec-kit/SPEC.md §The amendment lifecycle):
feature vs debt is the new-names litmus — a task adding any name to a
governed surface (script, knob, file/dir convention, tag, cross-component
contract) is a feature: write the SPEC amendment now (scope is the
spec-writing stage) and promote it `[spec:]`; no new names ⇒ debt. A ruling
longer than a few lines drafted into a queue entry is an amendment hiding
from `check-amendment-queue` — move it into one.

When done, name the iteration (no confirmation needed): replace `—` in the
header AND in the scope stamp — they ride in one commit. Do **not** flip the
`[stage:]` line; the arriving stage's skill flips it as its first step.

**Close by recommending the next stage.** Default `build`; recommend `align`
when one of its triggers fired (see `.claude/commands/align.md`). Trigger 3
is mechanical: an amendment left on disk whose component span is ≥2 kits
makes `check-stage-entry` assertion C red a build entry that lacks a
`<iter> align` stamp or a user-ruled `align-waived` line — so when this
scope promoted such an amendment, align is not optional, only waivable.
