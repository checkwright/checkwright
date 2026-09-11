# SPEC amendment: surface-ratchet

**No delta is applied; every replacement passage below is Not yet applied.**

This amendment serves `always-loaded-brevity-reach`. It is the gate half of the
`instruction-surface-tier` set: the ratchet that holds what the sweep (SPEC-sweep.md) and the
always-loaded rewrite (SPEC-resident-substrate.md) cut. It edits no `CLAUDE.md` prose; the
restating bullets the entry found are that rewrite's deltas.

**Reach — operator direction, 2026-09-12:** the ratchet holds the always-loaded files plus the files
a trigger loads whole (templates, agent definitions, binding shims). Kit SPECs stay under close's
growth walk.

**Merge text is phrase-shaped, and existing sentences are re-phrased rather than appended to.** That
follows the same direction: SPECs are not expected to grow each iteration, and brief instructions
read better to an LLM than verbose ones. Its reach past this set is filed through the gap inbox.

## The grounds this design rests on

This section is amendment-only rationale and does not merge. Every claim in it was probed on
2026-09-12 at the commit that stamped this stage.

**The reach gap, re-measured.** `check-brevity` reads one section of one file: this repo's
`CONTEXT_KIT_BREVITY_SECTION` resolves to lines 108–114 of a 204-line `CLAUDE.md`. Its test —
over budget *and* citing a deeper doc — is a per-bullet quality check, not a reach over growth. That
gate is unchanged; the reach is added beside it.

**Why a ratchet and not a budget.** A whole-file cap either strangles a dense file or never fires,
and per-section budgets need a maintained roster. A ratchet needs no budget: it reds when a surface
grows past its committed ceiling without a deliberate re-stamp, so growth shows in the commit that
causes it.

**Why per file, and only the held surfaces.** A trigger loads a template or an agent definition
whole, so the file is the unit of cost a reader pays. A SPEC is read by section through index-first
reading, so a whole-file ceiling on it would price a cost no reader pays. Close's growth walk reads
every SPEC that grew. What a re-stamp costs on the held surfaces, since 2026-08-22 (20 iteration
boundaries):

| Surface class | Commits touching it | Commits growing it | Re-stamps per iteration |
| --- | --- | --- | --- |
| `CLAUDE.md` | 43 | 15 | under one |
| Templates, agent definitions, binding shims | 86 | 72 | about four |

**Why the ceilings get their own file.** `--update-baseline` rewrites `always-loaded-baseline.txt` as
a header and one row (`native/src/emit/always_loaded.rs`, `update_baseline`), and that row's commit
anchors `--growth`. Ceiling rows in that file would be erased at every close, and a mid-iteration
re-stamp there would move close's growth anchor.

**Why rows move by exactly two acts.** `--ceiling` rewrites every row to current size: it raises a
grown file, which is the deliberate act, and locks any cut made at the same time. `--update-baseline`
does the same wherever a ceiling file exists. The gate never writes. It runs pre-commit on every
commit that couples a governed file, so a later re-stamp moves rows only for that commit's own
change.

**Why an absent row reds and a stale row does not.** A newly governed file is growth from nothing. A
row whose file is gone is a narrowing, and a ratchet that reds when a template is deleted would add a
violation by removing a surface.

**Why `on-surface`.** `init` writes no ceiling file (installer/README.md §What init seeds; only
context-kit's smoke runs `--update-baseline`). gate-sdk/SPEC.md §The install disposition reserves
`zero-config` for a gate reading a surface `init` writes.

**Why the load-triggered pathspec knob defaults empty.** Which files a harness loads whole depends on
the adopter's layout. A default spelling the kits' own template paths would red an adopter's
re-vendor commit for growth the kit shipped.

**Why the hook body stays outside.** Its lines are consumer state, not authored text, so a ceiling on
it would red on queue motion.

**Limits.** A re-stamp makes growth deliberate, not justified; whether it is justified stays close's
judgment. Load-triggered pathspecs outside the descriptor's static `couples=` list are held by the
battery, not by the generated hook — the class `couples-dynamic-root-resolution` already files.

## What changes

### (1) The surface ratchet gate

A new gate, `check-surface-ratchet`, holds every governed surface at or below its committed ceiling
{design-bearing}. It is a native module with a `.gate` descriptor (`# install: on-surface`,
`tier=precommit`, coupling the ceiling file, `CLAUDE.md`, the kit templates, `.claude/agents/*.md`
and `.claude/commands/*.md`) and a `good/`+`bad/` fixture pair:

- **Bad fixture:** one file over its ceiling, and one governed file with no row.
- **Good fixture:** one file at its ceiling, one below it, and a row naming an absent file.
- **Refusal axis:** a bespoke test beside the pair, on `check-brevity.test.sh`'s precedent.

`context-kit/SPEC.md` gains §The surface ratchet after §The brevity gate. **Not yet applied:**

> `check-surface-ratchet` — no governed surface above its committed ceiling.
>
> - **Governed:** `CONTEXT_KIT_SURFACES`, plus tracked files matching `CONTEXT_KIT_RATCHET_PATHS`;
>   size = newline count, the meter's measure.
> - **Ceilings:** `CONTEXT_KIT_CEILING_FILE` — a `# contract:` header, then `<lines> <path>` per
>   governed file, sorted by path.
> - **Red:** a file above its row, or a governed file with no row (a new surface grows from nothing).
>   The report names file, size and ceiling, and prints `--emit always-loaded --ceiling`, committed
>   with the growth.
> - **Clean:** otherwise. A row for a deleted or ungoverned file is ignored, so narrowing never reds.
> - **Exit 2:** ceiling file absent, a row unparsable, a knob unresolved.
> - **Writers:** never the gate — `--ceiling` in the growing commit, `--update-baseline` at close where
>   the file exists.
> - **Why a ratchet:** no budget to calibrate; growth shows in the commit that causes it. A re-stamp
>   makes it deliberate, not justified; close judges.
> - **Why these files, per file:** a trigger loads each whole. A SPEC is read by section; close's
>   growth walk reads it.
> - **Why its own file:** `--update-baseline` rewrites the baseline as one row, and that row anchors
>   close's growth read.
> - **Why `on-surface`:** `init` writes no ceiling file; `--ceiling` arms the gate.
> - **Outside:** the hook body — consumer state, not authored text.
> - **Limit:** pathspecs outside the descriptor's static `couples=` are held by the battery, not the
>   hook.

### (2) The meter gains `--ceiling`, re-phrased into its existing mode roster

Three edits to §The always-loaded meter {design-bearing}. The arm's usage line names the new operand.
**Not yet applied:**

- The closed operand set gains one mode bullet:

  > - **`--ceiling`** — rewrite the ceiling file (§The surface ratchet) to current sizes, creating it
  >   if absent; the baseline row stays. Checked write; exit 2 names the path.

- The `--update-baseline` bullet's opening is re-phrased in place:

  > *rewrites the baseline file, and the ceiling file where one exists — a close-stage act, …*

- The sentence counting the knobs the arm resolves is re-phrased to name them, adding
  `CONTEXT_KIT_CEILING_FILE` and `CONTEXT_KIT_RATCHET_PATHS` and dropping the count.

### (3) Two knobs

`context-kit/lib/context.sh` defaults two knobs, listed in §Layout and configuration and in
`templates/context-config.sh` {mechanical}:

- `CONTEXT_KIT_CEILING_FILE` — the ratchet's ceilings; default
  `${GATE_SDK_WORKFLOW_DIR:-.workflow}/surface-ceiling.txt`.
- `CONTEXT_KIT_RATCHET_PATHS` — git pathspecs for load-triggered surfaces; default empty
  (always-loaded files only).

This repo's `scripts/context-config.sh` sets
`CONTEXT_KIT_RATCHET_PATHS=('*/templates/*.md' '.claude/agents/*.md' '.claude/commands/*.md' ':(exclude)*/gate-tests/*')`.

### (4) This repo arms the ratchet after the cuts land

Arming happens in this order, once SPEC-sweep.md's and SPEC-resident-substrate.md's deltas have landed
{mechanical}:

- `scripts/gates.list` registers `check-surface-ratchet`.
- `--emit always-loaded --ceiling` writes `.workflow/surface-ceiling.txt`, which is committed, so the
  first ceiling is the cut state.
- The gate's name is appended to `.workflow/tightened-gates.txt`, because a vendored tree can run it.
- The generated pre-commit hook, the coupling graph and `docs/enforcement.md` are regenerated.

### (5) The close brevity pass names the ceiling file, re-phrased in place

`context-kit/templates/close-brevity.md` step 5 and §The close-stage brevity pass name the ceiling
file beside the baseline file {mechanical}. **Not yet applied**, step 5:

> 5. **Re-baseline and commit.** Finish with
>    `bash gate-sdk/bin/run-gates.sh --emit always-loaded --update-baseline` (it also lowers armed
>    ratchet ceilings) and commit the baseline and ceiling files, so next iteration measures from this
>    close.

In `doctrine-kit/DOCTRINE.md` rule 5, the *Enforced by:* line's pointer is re-phrased to name
§The brevity gate and §The surface ratchet together.

### (6) The consumer smoke exercises the ratchet

`context-kit/smoke/install.sh` registers the gate and runs `--ceiling`, then asserts in order: clean,
red once a surface grows, clean after a second `--ceiling` {mechanical}.

## Producers and consumers

- **The ceiling file.**
  - Producers: `--ceiling`, run by a session that grows a surface and by this repo's arming step
    (delta 4); `--update-baseline` at close, where the file exists (delta 2).
  - Consumer: `check-surface-ratchet`, through the generated hook and the battery.
  - Fields: `<lines>` is read by the gate's comparison; `<path>` by its match against the governed
    set. The header is read by `check-workflow-tiering` (checked-projection `# contract:` pointer) and
    resolved by `check-spec-pointer`.
- **`CONTEXT_KIT_CEILING_FILE` and `CONTEXT_KIT_RATCHET_PATHS`.** Both are read by the gate and the
  meter arm, and declared in both members' knob lists, so the config bridge resolves them from
  `lib/context.sh`.
- **The gate's report.** Read by the committing session; the help line names `--ceiling`.
- **Enabling config, actually set.** This repo sets the pathspecs (delta 3) and registers the gate
  (delta 4); the consumer smoke registers and seeds it (delta 6).
- **Narrowing (point 5).** No delta narrows a corpus. How the gate treats a narrowed governed set is
  delta 1's ignored stale row.

## Existing sections updated

- `context-kit/SPEC.md` §The always-loaded meter — the new mode, the re-phrased baseline bullet, the
  knob sentence (delta 2).
- `context-kit/SPEC.md` §Layout and configuration — layout rows (delta 1) and knob roster (delta 3).
- `context-kit/SPEC.md` §The close-stage brevity pass (delta 5).
- `context-kit/SPEC.md` §Testing — the fixture pair and refusal test (delta 1), the smoke steps
  (delta 6).
- `context-kit/templates/close-brevity.md` — step 5 (delta 5).
- `doctrine-kit/DOCTRINE.md` rule 5, Always-loaded shape — *Enforced by:* (delta 5).
- `context-kit/README.md` — its gate roster (delta 1).
- `docs/context-kit/SPEC.md`, `docs/doctrine-kit/DOCTRINE.md`, `docs/enforcement.md`,
  `docs/footprint.md` and `docs/value.md` — generated (all deltas).

## Retired spellings

- None — no name is retired; the operand set and the knob list only grow.

## Definition of Done

- [ ] **Causal completeness** — the ceiling file, both knobs and the gate report each have a named
      producer, consumer and field readers; enabling config is set in this repo and in the smoke.
- [ ] **Instruction surfaces: instruction only** — delta 5's template text is instruction only.
- [ ] **Merged by re-phrasing** — existing meter, pass and doctrine sentences re-phrased in place;
      §The surface ratchet stays phrase-shaped (operator direction, 2026-09-12).
- [ ] **Merged with no information lost** — §The surface ratchet reads whole without this file.
- [ ] **Amendment deleted** — this file removed on merge; `ls context-kit/SPEC-*.md` empty.
- [ ] **Removals propagated** — `check-amendment-retired-spelling` green.
- [ ] **Gaps filed** — any gap found during the work goes through the gap inbox.
