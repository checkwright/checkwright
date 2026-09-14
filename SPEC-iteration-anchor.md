# SPEC amendment: iteration-anchor

Queue entries: `drift-baseline-unnamed-iteration` and `always-loaded-baseline-restamp-unforced`,
ruled together as both bodies ask. Both are in unit set `evidence-population-fidelity` under
**operator direction, 2026-09-14, lead-relayed**. The amendment spans lifecycle-kit, drift-kit,
context-kit and the crate, so it sits at the repo root.

## The question, and the figures re-measured

Two readings answer "since this iteration started", and both get the start wrong without saying so.

- **`--emit drift-report`'s iteration-start commit** reads the queue header's name and runs
  `git log -S"<name> scope " -- <workflow-dir>/WORKFLOW-STATE.txt`, then takes the last line.
- **The always-loaded meter's delta and `--growth` worklist** are measured against the commit the
  baseline row records. That row is right only if the previous close re-stamped it.

**Re-measured at this stage (2026-09-14), on this iteration's own tree:**

- **The pickaxe is wrong after naming too, not only under the sentinel.** The report header reads
  `[iteration start 5fdbe54a]`. That is the scope commit that *named* the iteration, and
  `git log -S'evidence-population-fidelity scope '` returns only that commit. `--enter-stage
  --rename` rewrites column 1 of every data line, so the pickaxe's first match is the rename and
  never the boundary. Scope's own inbox drain (`c7d37260`) and survey record (`dd576b7a`) land
  between the boundary `58ef6cbb` and that rename. So every since-iteration KPI drops scope's
  intake for the rest of the iteration. Under the sentinel, `git log -S'— scope '` matches 294
  commits.
- **A fourth instance of the stale baseline, live.** The row names `9bfa332b` and was last written
  by `414648e2` (2026-09-12). Six scope boundary commits have landed since (`d982c91b`,
  `e6d62061`, `011ca0d0`, `f8983a85`, `25519e8d`, `58ef6cbb`). The bare meter prints
  `-21 since 9bfa332b`, and `--growth` prints `20 file(s) grew, +1439 net line(s)`. Measured
  against this iteration's start, the growth over the same governed prose is zero.
- **Why the re-stamp is skipped: no surface the closing session loads names it.**
  lifecycle-kit/templates/stages/close.md step 10 (the brevity pass) never mentions
  `--update-baseline`. This repo's `.claude/commands/close.md` does not splice
  `context-kit/templates/close-brevity.md`, which is the one surface whose last step does. The
  re-stamp is left to the session's memory.
- **The anchor is already written down and already gated.** After the boundary truncation, the
  state file's first data line is the boundary stamp. Its `<head>` field is the commit
  `--enter-stage` read at that moment, and `check-stage-evidence`'s stamp-provenance assertion
  requires it to equal `HEAD` when it is introduced. This iteration's first stamp carries
  `d57c4fec`, the parent of `58ef6cbb`. Between those two commits the queue differs only in its
  header line, which lies outside every pool a KPI reads.

## The ruling

Candidates for the drift-report anchor:

- **Refused: report `n/a` while the sentinel is live.** The report goes blank at the scope stage,
  which is the stage that reads the trend to pick units. It also leaves the post-rename shape
  above untouched.
- **Refused: the newest boundary-reset commit.** It is the right commit, but finding it costs a
  history walk keyed on a commit-message or diff shape, and a file the gates already read holds
  the same fact.
- **Taken: the first data line's `<head>`.** It is one file read with no history walk. It is
  immune to the sentinel and to the rename, because neither touches field five. It is also held
  true at introduction by a gate that already exists.

Candidates for the always-loaded baseline:

- **Refused: a gate asserting the baseline is recent.** That gates a cadence rather than a
  property, and it would red on a close that legitimately had nothing to re-stamp.
- **Refused as the whole fix: folding the re-stamp into the brevity commit.** A close that skips
  the pass skips the fold with it, and that is the attested failure. The instruction half *is*
  taken (delta 5), but as a pointer on the executed surface, next to a reading that stays honest
  when the instruction is missed.
- **Taken: the meter reads the same anchor.** `--growth` measures from the iteration-start commit,
  so the worklist is right whatever the baseline says. The bare line carries both deltas and
  flags a baseline that does not describe the surface the iteration opened with.

**One anchor, one reader in the crate, a knob per kit.** The contract belongs to lifecycle-kit,
which owns the stamp grammar. The read lives beside the cursor derivation in the crate. drift-kit
and context-kit each resolve their own state-file knob and hand the path in. That is the
`DRIFT_KIT_STATE_FILE` precedent, and it is how `native/src/sessions.rs` shares a derivation
below the config layer.

## The seam

- **Kit mechanism:** the anchor definition and its crate read, drift-report's use of it, the
  meter's figures, modes and staleness marker, `CONTEXT_KIT_STATE_FILE`, and the brevity
  template's wording.
- **Consumer config:** each kit's state-file knob, and this repo's `.claude/commands/close.md`
  pointer from step 10 to the brevity template. A binding is consumer config, and the kit's close
  template gains no slot (delta 5 says why).
- **Private rule content:** none in reach.

## What changes

### (1) The iteration-start commit is the first stamp's `<head>`, read in one place {design-bearing}

**Not yet applied.** In lifecycle-kit/SPEC.md §The state machine, after the paragraph ending
`every positional reader of fields one to four is unmoved.`, insert:

> **The iteration-start commit is the first data line's `<head>`.** The boundary truncation
> leaves the boundary stamp as the file's first data line, and that stamp's `<head>` is the commit
> the iteration opened on. Anything a reader wants to measure "since the iteration started" is
> measured from it. It is unaffected by the unnamed-iteration sentinel and by
> `--enter-stage --rename`, since both touch column 1 only. It is held true at introduction by
> §check-stage-evidence's stamp-provenance assertion. There is **no** iteration-start commit when
> the file is absent, when it has no data line (the no-cursor window), when that `<head>` is
> `none`, or when this clone cannot resolve it. Each reader states what it does then.

In lifecycle-kit/SPEC.md §lib/stages.sh, after the cursor-derivation paragraph, **not yet
applied**:

> The crate counterpart of these adapters, `native/src/stages.rs`, also holds the
> **iteration-start read**. It takes a state-file path and returns the first data line's
> `<head>` exactly as recorded, or empty in every no-commit case §The state machine lists. It
> verifies the commit resolves in one `git rev-parse --verify <head>^{commit}` before returning.
> A cross-kit reader hands in the path its own knob resolved, so it calls the crate read without
> gaining a lifecycle-kit vendoring dependency (the shared-derivation shape of
> §bin/session-id.sh).

In `native/src/stages.rs`: a pure function beside `data_lines` returns field five of the first
data line when it is 7 to 40 lowercase hex characters. A path-taking function reads the file,
applies that, and verifies with the one `rev-parse`. Unit tests cover the absent file, the
no-data-line window, `none`, a four-field line, a well-formed head that fails to resolve, and a
first line differing from the last. The last case is what pins *first* rather than cursor.

### (2) drift-report anchors on the first stamp through its own state-file knob {design-bearing}

`native/src/emit/drift_report.rs`: `iteration_start` stops reading the queue header and running
the pickaxe. It calls delta 1's read on `DRIFT_KIT_STATE_FILE`, which the arm already resolves
through its `DRIFT_KIT_*` family. `GATE_SDK_WORKFLOW_DIR` leaves the arm's `KNOBS` roster,
because it has no other reader in the arm. The unit test that pinned the name parse is replaced by
a test that pins the value reaching `Ctx`.

**Not yet applied.** In drift-kit/SPEC.md §The report skeleton, step 2's **Priced** sentences,
from `**Priced, because the position is not free:**` through `per-session cost rather than a
per-report one.`, become:

> The commit is lifecycle-kit's **iteration-start commit** (lifecycle-kit/SPEC.md §The state
> machine), read through that section's shared crate read on `DRIFT_KIT_STATE_FILE`. It is
> **never** a history search keyed on the iteration's name. The name is not unique while the
> unnamed-iteration sentinel stands, and `--enter-stage --rename` rewrites every stamp's name, so
> a search on it lands on the oldest sentinel commit or on the rename. **Priced, because the
> position is not free:** deriving it above the loop runs it in `--trend` mode too, so one
> state-file read and one `git rev-parse` run on every session start through the context hook.

**Not yet applied.** In drift-kit/SPEC.md §The KPI plugin contract, the sentence
`A plugin reading it gets the empty string when no baseline is derivable` becomes
`A plugin reading it gets the empty string when there is no iteration-start commit
(lifecycle-kit/SPEC.md §The state machine lists the cases)`.

**Not yet applied.** In drift-kit/SPEC.md §Bundled KPIs, `kpi-queue-net-delta`'s
`With no baseline (a standalone run, a fresh clone)` becomes
`With no iteration-start commit (no state file, the no-cursor window, or a head this clone cannot
resolve)`.

**Not yet applied.** In drift-kit/SPEC.md §Layout and configuration, `DRIFT_KIT_STATE_FILE`'s
first sentence gains a second reader: `…the stage-economics join reads for stamps (§The
stage-economics meter, history ∪ live), and whose first stamp the report skeleton reads for the
iteration-start commit (§The report skeleton)`.

### (3) The always-loaded meter measures growth from the iteration start and flags a stale baseline {design-bearing}

`CONTEXT_KIT_STATE_FILE` is new, and it is defined in `lib/context.sh` with the default
`${GATE_SDK_WORKFLOW_DIR:-.workflow}/WORKFLOW-STATE.txt`. It joins the meter arm's `KNOBS`.

`native/src/emit/always_loaded.rs`:

- `measure` takes the iteration-start commit as an argument, and the arm passes delta 1's read on
  its own knob. `Measurement` gains three figures: `start_commit` (empty when there is none);
  `start_surface`, the summed newline count of each `CONTEXT_KIT_SURFACES` path at that commit
  through `git show <commit>:<path>`, where a path git cannot show counts zero, as an absent
  surface does in the live measure; and `base_surface`, the baseline row's second field, which
  `split_row` parses today and discards. A row whose second field is not a bare non-negative
  integer gives no `base_surface`, on the same rule `base_total` follows.
- **Stale** means `base_surface` and `start_surface` are both present and differ. The baseline
  then does not describe the surface the iteration opened with. The test covers the surface half
  only, because the hook body's size at a past commit is not recoverable. That limit is stated in
  the SPEC.
- **The bare line** appends ` · surfaces <±n> since iteration start <start8>` when a start commit
  exists, and ` (baseline stale)` when stale. With no start commit the line is byte-identical to
  today's, which keeps a consumer without lifecycle-kit on today's reading.
- **`--growth`** diffs against the start commit when one exists, with the header
  `growth since iteration start <start8>: …`. Otherwise it diffs against the baseline commit
  exactly as today. When stale, a second header line reads `baseline <base8> is stale: its
  surfaces count <b>, the iteration opened at <s> — re-stamp with --update-baseline at close`.

**Not yet applied.** In context-kit/SPEC.md §The always-loaded meter, `--growth`'s bullet sentence
`It is derived from git against the commit the baseline already records, so it adds no second
baseline; a baseline commit git cannot resolve prints a one-line notice and exits clean.` becomes:

> It measures from lifecycle-kit's **iteration-start commit** (lifecycle-kit/SPEC.md §The state
> machine), read through that section's shared crate read on `CONTEXT_KIT_STATE_FILE`. So the
> worklist is this iteration's growth whether or not the last close re-stamped. Where there is no
> iteration-start commit it measures from the commit the baseline row records, and a baseline
> commit git cannot resolve then prints a one-line notice and exits clean. Neither path adds a
> second baseline file.

**Not yet applied.** In the same section, the **Default invocation** bullet becomes:

> - **Default invocation** prints one line: total, per-part breakdown, the delta against the
>   baseline when one exists, and — where an iteration-start commit exists — the surfaces' delta
>   since it. The line marks the baseline **stale** when the row's surface count differs from the
>   surfaces' size at the iteration-start commit, which means a close skipped its re-stamp and the
>   baseline delta is cumulative. The mark covers the surface half only: the hook body's size at
>   a past commit is not recoverable, so a stale hook half is not detected.

**Not yet applied.** In context-kit/SPEC.md §The surface ratchet, the bullet
`- **Why its own file:** \`--update-baseline\` rewrites the baseline as one row, and that row
anchors close's growth read.` becomes
`- **Why its own file:** \`--update-baseline\` rewrites the baseline as one row, whose surface count
is the meter's staleness witness.`

**Not yet applied.** In context-kit/SPEC.md §Layout and configuration, after
`CONTEXT_KIT_BASELINE_FILE`'s bullet:

> - `CONTEXT_KIT_STATE_FILE` — the lifecycle state file whose first stamp names the
>   iteration-start commit the meter's `--growth` and staleness read (§The always-loaded meter);
>   default `${GATE_SDK_WORKFLOW_DIR:-.workflow}/WORKFLOW-STATE.txt`. context-kit's own knob
>   rather than an import of lifecycle-kit's, so a consumer without lifecycle-kit resolves an
>   absent file and keeps the baseline-only reading.

### (4) kpi-always-loaded hands in the report's anchor and carries the mark {mechanical}

`native/src/emit/kpi/always_loaded.rs` calls `measure(&ctx.iteration_start)`. That is the same
value every member reads, and it adds no `CONTEXT_KIT_*` knob to drift-report's roster. The full
row is `line(&m)`, so it carries delta 3's additions unchanged. The `--trend` fragment appends
` stale` when stale and is otherwise unchanged.

**Not yet applied.** In drift-kit/SPEC.md §Bundled KPIs, `kpi-always-loaded`'s first sentence
becomes:
`the standing per-session surface: level, since-baseline delta, and the surfaces' delta since the
report's iteration-start commit, with the meter's stale-baseline mark, from context-kit's meter,
read **in process** as figures.`
Add to the same bullet: `It hands the meter the report's own \`DRIFT_KIT_ITERATION_START\`
rather than letting it resolve \`CONTEXT_KIT_STATE_FILE\`, so the row and the header can never
name two different starts. \`--trend\` appends \`stale\` to its \`loaded\` fragment when marked.`

### (5) The brevity procedure names the start, and this repo's close points at it {mechanical}

**Not yet applied.** `context-kit/templates/close-brevity.md` step 1's last two sentences become:
`Then run it with \`--growth\`: every governed prose file that grew net since the iteration
started, largest first. Both lists are the worklist; a \`stale\` baseline means the delta is
cumulative, so read the growth list for this iteration's share.` Step 2's lead
`Walk the growth since baseline` becomes `Walk the growth`.

**Not yet applied.** In `.claude/commands/close.md`'s **housekeeping** binding, after the
sentence ending `with the cause when one moves the wrong way.`:

> The template's brevity pass (step 10) is context-kit/templates/close-brevity.md, run whole —
> its re-baseline step included.

**No close-template slot is added, on purpose.** A new binding slot on
lifecycle-kit/templates/stages/close.md would red every vendoring consumer until bound
(`check-skill-binding`), which is a tightened gate for a kit-optional procedure. lifecycle-kit
also ships without context-kit, so its template cannot name that kit's template itself.

### (6) Release declarations {mechanical}

Append to `.workflow/release-declarations.md` `## Behavior changes`:

- `` **`--emit drift-report`** `` — the iteration-start commit is read from the state file's
  first stamp (`DRIFT_KIT_STATE_FILE`) instead of a pickaxe on the iteration name, so
  since-iteration KPIs include scope's own commits and stop anchoring on the oldest sentinel
  commit. A shallow clone without the stamped commit reports `n/a (no iteration baseline)`.
- `` **`--emit always-loaded`** `` — `--growth` measures from the iteration-start commit where the
  new `CONTEXT_KIT_STATE_FILE` resolves one, and the bare line adds the surfaces' delta since it
  and a `(baseline stale)` mark. Without a lifecycle state file the output is unchanged.

## Producers and consumers

- **The iteration-start commit.** Its producer is `--enter-stage`'s boundary stamp, whose
  `<head>` is enabled on every stage entry by default, with no config. It is read by delta 1's
  crate read and consumed by drift-report (delta 2) and by the meter arm (delta 3). The meter
  inside drift-report gets it through `Ctx` (delta 4). The report header, `DRIFT_KIT_ITERATION_START`
  and `kpi-queue-net-delta` read it at report time, every mode. `--growth`'s diff and the
  staleness test read it at meter time.
- **`CONTEXT_KIT_STATE_FILE`.** Its producer is `lib/context.sh`'s default, resolved by the config
  bridge, so it is on wherever context-kit is vendored. Its one reader is the meter arm, when it
  calls delta 1. drift-report does not declare it (delta 4).
- **`Measurement.start_commit`.** Readers: `line` (the suffix) and `growth` (the diff base).
- **`Measurement.start_surface`.** Readers: `line` (the delta) and the staleness test.
- **`Measurement.base_surface`.** Reader: the staleness test only. It is parsed from a field the
  row already carries, and `update_baseline` keeps writing it.
- **The stale mark.** Its producer is the staleness test. Readers: the bare line, `--growth`'s
  second header line, `kpi-always-loaded`'s trend fragment, and the close session reading
  `close-brevity.md` step 1 (delta 5).
- **Narrowing, and each reader's red condition.** Delta 2 drops `GATE_SDK_WORKFLOW_DIR` from one
  arm's declared roster. The roster's reader is the arm table's `knobs` query
  (`native/src/emit/mod.rs`), which the front-end's config bridge calls to decide what to
  resolve. Two things can go red. The bridge refuses a declared name the owning library does not
  define, and a removal cannot trigger that. `walk::knob_scalar` refuses a name the arm reads but
  did not declare, and the read is removed together with the declaration. That reader is
  monotone, so it clears by inspection. Build still runs the battery and the crate tests for any
  roster assertion this probe did not find: the generated pre-commit hook carries no
  drift-report line, and nothing else was found reading this arm's roster. Delta 3 narrows
  nothing. With no iteration-start commit every path is byte-identical to today's, so the crate's
  existing always-loaded unit tests and the consumer smoke's `--update-baseline` assertion, which
  reds on an absent baseline file, see unchanged input.

## Existing sections updated

- `lifecycle-kit/SPEC.md` — §The state machine and §lib/stages.sh (delta 1).
- `native/src/stages.rs` — the iteration-start read and its tests (delta 1).
- `native/src/emit/drift_report.rs` — `iteration_start`, `KNOBS` and its test (delta 2).
- `drift-kit/SPEC.md` — §The report skeleton, §The KPI plugin contract, §Bundled KPIs'
  `kpi-queue-net-delta` and §Layout and configuration (delta 2), and §Bundled KPIs'
  `kpi-always-loaded` (delta 4).
- `context-kit/lib/context.sh` — `CONTEXT_KIT_STATE_FILE` (delta 3).
- `native/src/emit/always_loaded.rs` — `measure`, `Measurement`, `line`, `growth`, `KNOBS`,
  tests (delta 3).
- `context-kit/SPEC.md` — §The always-loaded meter, §The surface ratchet and §Layout and
  configuration (delta 3).
- `native/src/emit/kpi/always_loaded.rs` — the anchor hand-in and trend mark (delta 4).
- `context-kit/templates/close-brevity.md` — steps 1 and 2 (delta 5).
- `.claude/commands/close.md` — the housekeeping binding's step-10 pointer (delta 5).
- `.workflow/release-declarations.md` — two Behavior-changes bullets (delta 6).
- `docs/lifecycle-kit/SPEC.md` — generated mirror, regenerated (delta 1).
- `docs/drift-kit/SPEC.md` — generated mirror, regenerated (deltas 2 and 4).
- `docs/context-kit/SPEC.md` — generated mirror, regenerated (delta 3).

## Retired spellings

- None — no delta retires a literal spelling. The pickaxe is replaced in prose as well as code,
  but `git log -S` survives legitimately in queue-kit's and gate-sdk's unrelated recovery
  sections, and the sentences delta 2 rewrites are replaced in place.

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
- [ ] **Amendment deleted** — this file removed on merge; none remain at the
      repo root (`ls SPEC-*.md`).
- [ ] **Removals propagated** — every name this change retired is declared in
      `## Retired spellings` above, and `check-amendment-retired-spelling` runs
      each declaration against the whole tracked tree, not against the specs
      alone.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed as
      debt tasks (a build-time causal gap is resolved that session, not
      deferred).
