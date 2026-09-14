# SPEC amendment: overhead-self

Queue entry: `overhead-meter-resolves-the-newest-transcript-not-its-own`. It is in unit set
`evidence-population-fidelity` under **operator direction, 2026-09-14, lead-relayed**.

**The resolution rule at a delegated session is the lead's decision (2026-09-14, from the
iteration's intent oracle).** It is a decision, not an operator direction and not a ruling, so a
later scope or spec may revise it. The grounds relayed with it: this iteration makes each reading
correct or visibly absent, and only the required-operand shape is exact wherever it measures and
says so wherever it cannot. **Spec decided the top-level half alone:** it converges on the SPEC's
own sentence and changes no caller.

## The question, and the premise re-measured

drift-kit/SPEC.md §The overhead meter says a bare invocation resolves "the transcript the invoking
session is itself running in". The implementation takes the newest candidate transcript instead,
and that disagrees with the SPEC in two places.

- **A top-level session never uses its own identity.** `native/src/emit/overhead_meter.rs` passes
  `CLAUDE_CODE_SESSION_ID` to the shared resolver only when `CLAUDE_CODE_CHILD_SESSION` is set.
  With the flag unset it blanks the id and scans both tiers for the newest file. Any subagent the
  session dispatched, or a live one, out-mtimes it.
- **A delegated session cannot know its own transcript.** Probed 2026-09-14 in a lead-dispatched
  stage session:
  - The environment carries only the root's uuid and the child flag. No variable names the
    agent's own id.
  - Every subagent transcript at every depth sits flat under `<root>/subagents/`. On this
    machine's 1307 meta files, depth-1 transcripts carry no `parentAgentId` and all 695 deeper
    ones do.
  - The running Bash call is not yet in the session's own transcript: a unique token in the
    command had zero hits.
  - So the narrowed scan's newest-wins pick is a live grandchild as often as it is the session.
    That is the attested 2026-09-06 close reading (54 against 76 per cent governance).

## The decision

- **Refused: exclude descendants through the meta layer.** Two residuals print a plausible wrong
  total with nothing in the reading to show it: a lead's concurrent direct-child fork winning on
  mtime, and a session at depth 2 or more resolving to its ancestor.
- **Refused: anchor on the last lifecycle stamp.** It couples the meter to lifecycle stamps and
  misreads an ad-hoc child.
- **Taken: a delegated session passes the operand**, and the operand also accepts a stamp's
  `session8`. A bare invocation at a verified delegated session prints a notice at exit 0 and logs
  nothing. A top-level session resolves exactly through its harness id.

## The seam

- **Kit mechanism:** the delegation verdict, the three bare dispositions, the widened operand, and
  the kit templates' invocation wording.
- **Consumer config:** this repo's close binding, which passes the close stamp's id.
- **Private rule content:** none in reach.

## What changes

### (1) The delegation verdict is one shared function {design-bearing}

`native/src/sessions.rs` gains a function `delegation(&Inputs)` that returns one of three
verdicts. It is the child-flag verification `resolve` already performs inline, factored out so
the overhead meter reads the same verdict rather than a second copy of it.

- **top-level (with its id):** `CLAUDE_CODE_CHILD_SESSION` is unset and `CLAUDE_CODE_SESSION_ID`
  is set. Or the flag is set, the narrowed scan `<dir>/<uuid>/subagents/*.jsonl` is empty, and
  `<dir>/<uuid>.jsonl` exists; that is the spurious flag.
- **delegated:** the flag is set, the id is set, and the narrowed scan is non-empty.
- **undetermined:** no harness id, or a set flag with both the narrowed scan and the top-level
  transcript absent.

`resolve` calls it. Its behaviour, all three refusal texts, and the `--emit session-id` arm's
output are unchanged, and the existing session-id unit tests hold as they are.

**Not yet applied.** In lifecycle-kit/SPEC.md §bin/session-id.sh, append to the paragraph ending
`what lives there is the one implementation of it.`:

> The child-flag verification in source 3 is exposed from that module as its own verdict
> (top-level, delegated, undetermined), because drift-kit's overhead meter decides whether it may
> resolve at all on that verdict (drift-kit/SPEC.md §The overhead meter). It is one verification
> with two readers, never a second copy.

### (2) The meter measures exactly or says it cannot {design-bearing}

In `native/src/emit/overhead_meter.rs`, the bare invocation branches on delta 1's verdict:

- **top-level:** measure `<sessions-dir>/<uuid>.jsonl`. If that file is absent, print the existing
  no-transcript notice naming the path.
- **delegated:** measure nothing and log nothing. Print at exit 0:
  `overhead-meter: a delegated session cannot identify its own transcript — pass it: --emit
  overhead-meter <transcript.jsonl | session8>` followed by a `help:` line naming this session's
  stamp id as the usual operand.
- **undetermined:** keep the two-tier newest-candidate scan. The report's first line gains
  `(resolved by newest transcript — no harness session id)`, so a reading taken on the
  single-operator assumption says so. Spec calibrated this branch inside the decision's envelope:
  it reaches only a harness that exports no session id, which is the documented source-3 case of
  lifecycle-kit/SPEC.md §bin/session-id.sh.

**The operand widens.** An operand naming an existing file is a transcript path, as today.
Otherwise an operand of exactly eight characters is a `session8` and resolves through
`sessions::find`, the stage-economics meter's inverse lookup, which normalizes each candidate's
basename so a stamp's `<hex>` finds `agent-<hex>.jsonl`. Anything else, or a key with no match,
is the no-transcript notice at exit 0 naming the operand. `USAGE` names both forms. Unit tests
cover each bare verdict, including that the delegated verdict writes no log line, and both operand
forms plus the unmatched key.

**Not yet applied.** In drift-kit/SPEC.md §The overhead meter, the section's opening invocation
synopsis, `` `--emit overhead-meter [transcript.jsonl]` measures the methodology's own cost, `` becomes
`` `--emit overhead-meter [transcript.jsonl | session8]` measures the methodology's own cost, `` —
the widened operand this delta introduces two sentences later, so the section does not show two
different bracket notations for the same arm.

**Not yet applied.** In drift-kit/SPEC.md §The overhead meter, the opening paragraph's sentences
from `A bare invocation resolves` through `whose newest candidate wins.` become:

> `[transcript.jsonl | session8]` — the operand names the transcript by path or by a stamp's
> eight-character session id, resolved through the shared inverse lookup. A bare invocation
> measures **only a transcript it can identify**: a top-level session measures
> `<sessions-dir>/<harness-id>.jsonl` under `DRIFT_KIT_SESSIONS_DIR`. A **delegated** session — the
> shared verdict of lifecycle-kit/SPEC.md §bin/session-id.sh — prints a notice and logs nothing,
> because nothing in its environment names its own transcript and every descendant's transcript
> sits beside it. So a delegated caller passes the operand, usually its own stage stamp's id. A
> session whose harness exports no id falls back to the newest-candidate scan, and the reading says
> it did.

Two sentences are added after `a missing transcript is a 0-exit notice, not a failure.`, **not
yet applied**:

> The refusal at a delegated session is preferred over two ways of keeping the bare invocation:
> excluding descendants through the harness meta layer, and anchoring on the last lifecycle stamp.
> Both print a plausible wrong total in a case the reading cannot show.

The replacement text is undated on purpose. The decision's class and date stay in this amendment,
the queue entry and git history, per the provenance seam.

### (3) The callers pass the operand {mechanical}

- **Not yet applied.** `.claude/commands/close.md` housekeeping: `First meter this closing session
  with the \`--emit overhead-meter\` arm,` becomes `First meter this closing session with
  \`--emit overhead-meter <session-id>\`, passing this session's own \`close\` stamp id,`.
- **Not yet applied.** `drift-kit/templates/economics.md` step 1:
  `` `bash gate-sdk/bin/run-gates.sh --emit overhead-meter` on the closing session `` becomes
  `` `bash gate-sdk/bin/run-gates.sh --emit overhead-meter <session-id>` on the closing session,
  passing its stage stamp id (a delegated session must) ``.
- **Not yet applied.** `drift-kit/README.md`'s command block comment for the overhead meter
  becomes `# governance-vs-task byte proxy for this session's transcript (a delegated session
  passes its transcript or stamp id)`.
- `drift-kit/smoke/install.sh` already passes the fixture path, so it is unchanged. It stays the
  classifier oracle.

### (4) Release declaration {mechanical}

Append to `.workflow/release-declarations.md` `## Behavior changes`:

- `` **`--emit overhead-meter`** `` — a bare invocation inside a delegated session now prints a
  notice and logs nothing instead of measuring whichever transcript was newest. A top-level
  session measures its own transcript exactly. The operand also accepts a stamp's eight-character
  session id. Pass your session's transcript or stamp id wherever you meter from a dispatched
  session.

## Producers and consumers

- **The delegation verdict.** Its producer is delta 1's function, over environment the
  harness sets in every Bash call, so no config is needed. Its consumers are `sessions::resolve`
  (unchanged output) and the overhead meter's bare branch (delta 2).
- **The delegated notice.** Its producer is delta 2. Its consumer is the invoking session reading
  stdout at exit 0. It logs nothing, so `kpi-overhead`'s trailing window, which reads `pct`,
  `gate`, `total` and `date`, never receives a row measured off a sibling or descendant.
- **The `session8` operand.** Its producers are `--enter-stage`'s stamp field and the close
  binding (delta 3). Its consumer is `sessions::find`, an existing reader with an existing
  enabling path, the `DRIFT_KIT_SESSIONS_DIR` default.
- **The undetermined marker.** Its producer is delta 2. Its reader is the human reading the
  report. It adds no log field.
- **Narrowing, and each reader's red condition.** The log's population narrows: a delegated bare
  invocation writes no row. `kpi-overhead` degrades to its fail-visible `run --emit overhead-meter`
  row on an **absent** log, never on a short one, and its caveats print the session count. So
  fewer rows can make it neither red nor fail; it averages over fewer sessions and says how many.
  No gate reads the log. The drift-kit smoke asserts rows appended by a path operand, and that path
  is unchanged.

## Existing sections updated

- `native/src/sessions.rs` — the delegation verdict, with `resolve` calling it (delta 1).
- `lifecycle-kit/SPEC.md` — §bin/session-id.sh (delta 1).
- `native/src/emit/overhead_meter.rs` — bare dispositions, operand forms, `USAGE`, tests
  (delta 2).
- `drift-kit/SPEC.md` — §The overhead meter (delta 2).
- `.claude/commands/close.md` — the housekeeping binding's meter invocation (delta 3).
- `drift-kit/templates/economics.md` — step 1 (delta 3).
- `drift-kit/README.md` — the command block comment (delta 3).
- `.workflow/release-declarations.md` — a Behavior-changes bullet (delta 4).
- `docs/lifecycle-kit/SPEC.md` — generated mirror, regenerated (delta 1).
- `docs/drift-kit/SPEC.md` — generated mirror, regenerated (delta 2).
- `docs/drift-kit/README.md` — generated mirror, regenerated (delta 3).

## Retired spellings

- None — no delta retires a literal. The bare-invocation sentences are rewritten in place, and no
  knob, arm or file name changes.

## Definition of Done

- [ ] **Causal completeness** — every new state/event/interface has a named,
      reachable producer and a named consumer; every new field has a named
      reader at a named transition.
- [ ] **Instruction surfaces: instruction only** — replacement text for a
      template, agent definition or shim carries no grounds; a delta places them
      (the Content-tiering / SSOT rule).
- [ ] **Merged with no information lost** — each addition integrated into its
      proper canonical-spec section (not appended); the merged spec reads as one
      coherent document a reader who never saw the amendment can use alone.
- [ ] **Amendment deleted** — this file removed on merge; none remain for the
      component (`ls drift-kit/SPEC-*.md`).
- [ ] **Removals propagated** — every name this change retired is declared in
      `## Retired spellings` above, and `check-amendment-retired-spelling` runs
      each declaration against the whole tracked tree, not against the specs
      alone.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed as
      debt tasks (a build-time causal gap is resolved that session, not
      deferred).
