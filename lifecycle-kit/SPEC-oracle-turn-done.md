# SPEC amendment: oracle-turn-done-marker

Pairs `oracle-turn-done-marker`. A stage session stays resumable after its stage ends, as the split posture's intent oracle or to answer a routed question (§check-dispatch-entry, the workflow-state guard's third rule, names both). A resumed turn appends its answer to the stage's journal past the `DONE` its first report wrote, so `DONE` stops being the last line. lifecycle-kit/templates/lead.md's fallback completion read takes only the last line, so it reads that finished session as unfinished and falls back to asking it.

**What the tree does today**, read at spec:

- **The stage templates' last step names the act once.** Each of the six stage templates (`grep -l 'Append \`DONE\`' lifecycle-kit/templates/stages/*.md`: align, build, close, scope, spec, validate) ends with the one sentence "Append `DONE` as the file's last line before you report." It is headed *Last step*, so a resumed turn reads it as already discharged.
- **delegation-kit's contract already covers a resumed turn.** `templates/agent-execution.md`'s resume-journal bullet says "on success it appends a `DONE` marker" and "a session resumed past its own marker appends after it". A resumed turn that succeeds therefore owes the marker again. The gap is in the stage step's framing, so the fix lands in lifecycle-kit alone and delegation-kit takes no edit.
- **The lead's reader** is lead.md's sentence "The `DONE` check is valid only as the journal's **last line**: an out-of-order append silently invalidates it and leaves the marker visible."
- **`check-stage-skill-coverage`'s third direction matches the step's citation, never its sentence** (§check-stage-skill-coverage, *The marker is a citation, never a sentence*). Rewording the step's act moves no gate.

The entry gives two options: re-stamp `DONE`, or exempt oracle turns. Re-stamping is chosen, because an exemption would leave the lead's fallback read with no completion signal for any resumed session, the oracle included. No heading is added for the turn: a `DONE` mid-file already marks where a session's earlier turn ended (§The state machine, *One journal per stage*).

## What changes

### (1) The stage templates' last step covers a later turn {mechanical}

In each of `lifecycle-kit/templates/stages/{align,build,close,scope,spec,validate}.md`, the *Last step* paragraph's closing sentence "Append `DONE` as the file's last line before you report." becomes the sentence below, the same in all six. **Not yet applied.**

> Append `DONE` as the file's last line before you report, and again before the report of any later turn that appends to the file, as a resumed session answering a routed question does.

The paragraph's citations stay as they are, `lifecycle-kit/SPEC.md §The state machine` among them, which `check-stage-skill-coverage`'s third direction reads.

### (2) The lead's completion read names the re-stamp {mechanical}

In `lifecycle-kit/templates/lead.md`, the paragraph opening "**When the notification does not arrive**", the closing sentence "The `DONE` check is valid only as the journal's **last line**: an out-of-order append silently invalidates it and leaves the marker visible." becomes the sentence below. **Not yet applied.**

> The `DONE` check is valid only as the journal's **last line**, which a resumed session's turn writes again (each stage template's last step), so it answers for the newest turn, the intent oracle's included. An out-of-order append silently invalidates it and leaves the marker visible.

### (3) The SPEC's grounds {mechanical}

Each **Not yet applied.**

- `lifecycle-kit/SPEC.md` §templates/stages/, the paragraph opening "**The last step mirrors the first**": the sentence "It names one **act**, appending `DONE` as the file's last line before reporting, and cites the two owners, which keeps it far below `LIFECYCLE_KIT_SHIM_NGRAM`'s bar." becomes:

  > It names one **act**, appending `DONE` as the file's last line before reporting, again at any later turn that appends, and cites the two owners, which keeps it far below `LIFECYCLE_KIT_SHIM_NGRAM`'s bar. The later turn is named because a stage session stays resumable after its stage ends (§check-dispatch-entry, the workflow-state guard's third rule). Its answer lands in the journal past its first marker, where §templates/lead.md's fallback completion read takes the last line alone.

- `lifecycle-kit/SPEC.md` §The state machine, the paragraph opening "**One journal per stage**": "the last line answers *did the session that wrote last finish*" becomes "the last line answers *did the session that wrote last finish its turn*".

## Producers and consumers

- **The re-stamped marker** (deltas 1 and 2). Producer: a resumed stage session at the end of a turn that appended to its stage's journal, appending through the journal arm (§The journal arm), whose verbatim write lets `DONE` stand as the last line. The enabling deployment is the split posture's intent oracle and any lead routing a question to a resumed stage session, both in this repository's live use. Consumers:
  - lead.md's fallback completion read, the last line, when a completion notification does not arrive;
  - a cold reader under delegation-kit's contract (delegation-kit/SPEC.md §Resume journal — agent writes, scratch reset sweeps), where no `DONE` last means interrupted.
- **No gate reads the marker.** `--enter-stage`'s journal assertion reads written content and never `DONE` (§bin/enter-stage.sh), so a second marker in one file moves no verdict. `check-stage-skill-coverage` reads the citation, which delta 1 keeps.

Corpus probes: `grep -rn 'DONE' lifecycle-kit/templates lifecycle-kit/SPEC.md delegation-kit/templates delegation-kit/SPEC.md` for the readers and writers of the marker. `grep -rln 'Append \`DONE\`' --include=*.md .` over the tree found the six stage templates and no consumer binding restating the step.

## Existing sections updated

- `lifecycle-kit/templates/stages/align.md`, `build.md`, `close.md`, `scope.md`, `spec.md`, `validate.md`, the *Last step* paragraph (delta 1).
- `lifecycle-kit/templates/lead.md`, the paragraph opening "When the notification does not arrive" (delta 2).
- `lifecycle-kit/SPEC.md` §templates/stages/ and §The state machine (delta 3).
- `docs/lifecycle-kit/SPEC.md`, the generated mirror, regenerated with `bash gate-sdk/bin/run-gates.sh --emit docs-mirror --write` (delta 3).

## Retired spellings

- None — no delta retires a name; the rewritten sentences mint none.

## Definition of Done

- [ ] **Causal completeness** — every point of SPEC §The causal-completeness check holds for each new state, event, interface and obligation.
- [ ] **Instruction surfaces: instruction only** — replacement text for a template, agent definition or shim carries no grounds; a delta places them (the Content-tiering / SSOT rule).
- [ ] **Merged with no information lost** — each addition re-phrases the canonical-spec text it refines rather than appending to it; the merged spec reads as one document a reader who never saw the amendment can use alone.
- [ ] **Amendment deleted** — this file removed on merge; none remain for the component (`ls lifecycle-kit/SPEC-*.md`).
- [ ] **Removals propagated** — every name this change retired is declared in `## Retired spellings` above, and `check-amendment-retired-spelling` runs each declaration against the whole tracked tree, not against the specs alone.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed as debt tasks (a build-time causal gap is resolved that session, not deferred).
- [ ] **The local oracle** — the battery and the lifecycle-kit fixture suite green.
- [ ] **The Done move** lands before the drain stage, on the local oracle.
