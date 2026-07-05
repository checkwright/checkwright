The `scope` (design) stage of a Checkwright iteration (one iteration per kit,
in the extraction order README.md records). Read `EXTRACTION.md` (local-only
brief) and the platform sources for the kit being extracted; decide the kit's
layout, config surface, and de-hardcoding worklist; name the iteration after
the kit. Exit condition: the design is written down (this session's plan or
the kit SPEC draft) and the seam is ruled — what ships as mechanism, what
stays platform rule content, what becomes consumer config.

**First step — reset + stamp evidence.** `scope` is the iteration boundary:
truncate `.workflow/WORKFLOW-STATE.txt` back to its header (git history keeps
the prior iteration's stamps), append `— scope <session-id> <date>`, and set
the TASK-QUEUE.md header to `## Iteration: —  [stage: scope]` — the
arriving-stage flip, here bootstrapping an unnamed iteration. Take
`<session-id>` from `bash lifecycle-kit/bin/session-id.sh` (it reads the id
from the newest transcript — never hand-pick it); `<date>` is `date +%F`.

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
