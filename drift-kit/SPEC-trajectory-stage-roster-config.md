# SPEC amendment: trajectory-stage-roster-hardcode

<!-- Owning component drift-kit → drift-kit/SPEC.md. Paired with the New-Features
     entry [spec: SPEC-trajectory-stage-roster-config.md]. Delta only; merges
     into drift-kit/SPEC.md (§Layout and configuration, §The published-evidence
     extractor) and this repo's consumer config on completion, then this file is
     deleted. -->

`drift-kit/bin/trajectory.sh` renders the governed-trajectory table off a
**hardcoded five-stage roster** (`scope align build validate close`) at four
sites, so any stamp outside that literal is silently dropped: a `spec` stamp
falls through the `:52` case arm's `*) continue`, is filtered out by the `:68`
git-log grep pre-filter before the harvest loop, gets no slot in the `:162`
`render_stages` loop, and is unlabelled by the `:180` column-header literal
`stages (s a b v c)`. This repo's live roster is six stages
(`scope spec align build validate close`, `scripts/lifecycle-config.sh`), so the
extractor renders a 5-slot row with the `spec` stage invisible — and it is the
`/economics` read that motivated adding `spec` that the tool then under-reports.
The fix: read the **configured** roster, not a literal list.

## Seam ruling (settle first — the exit condition)

- **Mechanism (ships in drift-kit, generic):** a new config knob
  `DRIFT_KIT_STAGES` (ordered array of stage names). `trajectory.sh` reads it at
  all four sites. The knob follows the **established drift-kit seam convention**:
  drift-kit re-derives cross-kit facts with its own knob rather than importing a
  sibling kit's bin/lib contract — the identical pattern §Layout already
  documents for `DRIFT_KIT_SESSIONS_DIR` (lifecycle's cwd-slug derivation) and
  `DRIFT_KIT_STATE_FILE` (lifecycle's WORKFLOW-STATE path). drift-kit registers
  no gates and is advisory-standalone (the guard-kit precedent); it must stay
  vendorable without lifecycle-kit, which sourcing `LIFECYCLE_KIT_STAGES`
  directly would break. **Ruled out on the governed surface: (B) sourcing
  lifecycle-kit's `lib/stages.sh`** — a hard cross-kit code dependency that
  violates the documented convention and the standalone contract.
- **Fall-open default (kit default unchanged):** when `DRIFT_KIT_STAGES` is
  unset the knob defaults to the historical `(scope align build validate close)`,
  so standalone drift-kit and un-upgraded consumers emit **byte-identical**
  tables — the abbreviator (below) reduces to the current single-letter form
  when no roster collision exists. Same non-breaking principle
  `stage-posture-split` used to keep the kit default at five stages while a
  consumer activates a wider roster.
- **Consumer config (this repo activates):** `scripts/drift-config.sh` sets
  `DRIFT_KIT_STAGES` by **deriving from the sole roster owner** —
  it sources `scripts/lifecycle-config.sh` and assigns
  `DRIFT_KIT_STAGES=("${LIFECYCLE_KIT_STAGES[@]}")`. This keeps
  `lifecycle-config.sh` the single roster owner (derivation-first doctrine);
  **ruled out: a parallel literal roster in `drift-config.sh`**, which would
  reintroduce the very roster-drift class this task exists to eliminate. Both
  files are consumer-owned config in the same `scripts/` dir — this is
  config-to-config wiring, not a kit coupling.
- **Private rule content:** none. A stage roster is generic methodology, not a
  term list or product constant — nothing crosses the provenance seam.

## What changes

### The knob and its default

`trajectory.sh` sets a fall-open default after it sources its config (beside the
existing `DRIFT_KIT_TRAJECTORY_SURFACES` / `DRIFT_KIT_GATES_FILE` defaults):
`declare -p DRIFT_KIT_STAGES &>/dev/null || DRIFT_KIT_STAGES=(scope align build
validate close)`. A consumer overrides it in `drift-config.sh` (or via
`DRIFT_KIT_CONFIG_FILE`).

### The four roster-reader sites

1. **`:52` case arm** — the literal `scope|align|build|validate|close) ;;
   *) continue` becomes a membership test against `DRIFT_KIT_STAGES`: a stamp
   whose stage token is not a roster member is skipped, any member is harvested.
2. **`:68` git-log grep pre-filter** — the hardcoded alternation
   `(scope|align|build|validate|close)` is built from the roster at runtime
   (`IFS='|'; "${DRIFT_KIT_STAGES[*]}"`), so the pre-filter and the case arm read
   one source and cannot diverge.
3. **`:162` `render_stages`** — the loop `for s in scope align build validate
   close` iterates `"${DRIFT_KIT_STAGES[@]}"` instead; each configured stage
   renders present (its abbreviation) or absent (`·`), in roster order.
4. **`:180` column-header literal** — `stages (s a b v c)` is derived from the
   roster via the same abbreviator, so the header always legends the exact
   configured roster rather than a frozen five.

### Render: the shortest-unique-prefix abbreviator

The stage column is **space-joined tokens** (not a fixed-width grid), so a
variable roster length and a variable token width both render without layout
work — the column simply grows. Each stage's slot label is its **shortest prefix
unique among the roster**: for `(scope spec align build validate close)` that is
`sc sp a b v c` (scope and spec collide at one letter, disambiguate at two;
every other stage is unique at one). For the five-stage default the shortest
unique prefix is the single letter for every stage, so the header and cells are
**byte-identical to today** (`s a b v c`) — the collision-driven second
character appears only when the roster actually collides. The header (site 4)
and the cells (site 3) call the same abbreviator over the same roster, so they
cannot drift. This resolves both open questions the deferred entry named — render
width when the roster count varies (space-joined, no grid) and the label
collision the six-stage roster introduced (scope/spec).

## Producers and consumers (causal completeness)

New state/interface: the config knob `DRIFT_KIT_STAGES` (ordered stage-name
array). It is the only new field.

- **`DRIFT_KIT_STAGES`.** *Producer:* drift-kit's fall-open default in
  `trajectory.sh` (unconditional when unset) and `templates/drift-config.sh`
  (the documented, copy-and-override knob); this repo's `scripts/drift-config.sh`
  sets it by sourcing `scripts/lifecycle-config.sh` and copying
  `LIFECYCLE_KIT_STAGES`, so the value is *emitted everywhere it must be* — a
  bare run with no config still gets the five-stage default. *Consumers/readers:*
  all four `trajectory.sh` sites — the `:52` case-arm membership test, the `:68`
  grep-alternation builder, the `:162` `render_stages` loop, and the `:180`
  header derivation (via the abbreviator, which reads the whole array). Every
  reader is named and reachable; the field is read at every site, populated at
  none it is not read.
- **Whole-component survey — no other reader.** `bin/stage-economics.sh` reads a
  stamp's stage token **verbatim** (`while read -r iter stage session8`) and
  prices any stage, so it already renders `spec` correctly and needs no change;
  `bin/overhead-meter.sh` classifies bytes by path, not by a roster. The
  hardcoded roster lives **only** in `trajectory.sh`. lifecycle-kit owns
  `LIFECYCLE_KIT_STAGES`; nothing in drift-kit's component is touched beyond
  `trajectory.sh`, its SPEC, and its config template.

## Existing sections updated

At merge, integrate (do not append) into `drift-kit/SPEC.md`:

- **§The published-evidence extractor** — the "iteration + stages run" bullet
  says stamps are "rendered as fixed stage slots so a skipped stage reads as a
  gap"; update to "rendered as one slot per configured stage (`DRIFT_KIT_STAGES`,
  roster order), each labelled by its shortest roster-unique prefix, so a skipped
  or non-roster stage reads as a gap."
- **§Layout and configuration** — add the `DRIFT_KIT_STAGES` knob entry: the
  ordered stage roster the trajectory extractor renders; default
  `(scope align build validate close)`; a consumer deriving it from a sibling
  lifecycle config is the SSOT activation. Place it beside the other
  trajectory knobs (`DRIFT_KIT_TRAJECTORY_SURFACES`, `DRIFT_KIT_GATES_FILE`),
  and note it as the third instance of the re-derive-with-own-knob convention
  (alongside `DRIFT_KIT_SESSIONS_DIR` and `DRIFT_KIT_STATE_FILE`).

## Activation (this repo's consumer config — the build worklist)

- `drift-kit/bin/trajectory.sh`: add the `DRIFT_KIT_STAGES` fall-open default;
  add the abbreviator helper; rewrite the four sites to read the roster.
- `drift-kit/templates/drift-config.sh`: document the `DRIFT_KIT_STAGES` knob
  (commented, defaulted) so a consumer copy carries it.
- `scripts/drift-config.sh`: source `scripts/lifecycle-config.sh` and set
  `DRIFT_KIT_STAGES=("${LIFECYCLE_KIT_STAGES[@]}")` (lifecycle-config stays the
  sole roster owner; drift derives).
- Regenerate and re-gate the derived projections: `docs/evidence-data.md`
  (`trajectory.sh --emit` — `check-trajectory-fresh` byte-compares it; the
  six-stage header and the now-visible `spec` slot land here), and the
  render-fidelity projection `docs/drift-kit/SPEC.md`
  (`check-docs-render-fidelity`) if the SPEC edits move rendered lines.
- Run the full gate battery and drift-kit's fixture suite before committing.

## Definition of Done

- [ ] **Causal completeness** — `DRIFT_KIT_STAGES` has a named, reachable
      producer (fall-open default + template + this repo's derived activation)
      and named consumers (the four `trajectory.sh` sites); no field lacks a
      reader; the whole-component survey confirms `trajectory.sh` is the only
      roster-reader.
- [ ] **Merged with no information lost** — the knob lands in §Layout and the
      extractor-prose delta lands in §The published-evidence extractor; the
      merged SPEC reads as one coherent document.
- [ ] **Non-breaking default proven** — with `DRIFT_KIT_STAGES` unset the
      emission is byte-identical to the pre-change five-stage table.
- [ ] **`spec` now visible** — the re-emitted `docs/evidence-data.md` renders a
      `spec` slot; `check-trajectory-fresh` is green.
- [ ] **Amendment deleted** — this file removed on merge; none remain for
      drift-kit (`ls drift-kit/SPEC-*.md` empty).
- [ ] **Gaps filed** — any cross-component gap surfaced in build is resolved
      that session, not deferred.
