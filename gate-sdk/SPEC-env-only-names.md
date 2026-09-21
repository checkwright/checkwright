# SPEC amendment: env-only-names

`check-docs-cmd` counts a knob name as defined when it occurs in a tracked kit
source, or is one of `knobs::static_names()`. The payload withholds every kit's
`smoke/`. So a name whose only tracked kit-root occurrence is under `smoke/` is
defined in this tree and undefined in every adopter's. Three names are exposed, and
nothing reds today only because no adopter documents them. The entry asked whether a
smoke-only knob needs a shipped definition site at all. **This amendment rules it
by what the name is. A name a kit reads from, or hands through, the environment is
an environment-only name, and its site is its kit table's `env_only` list. A name
that exists only as smoke test data owes no site.**

**The three names, re-measured (2026-09-21).** Each one's only non-`.md` kit-root
occurrence is its kit's `smoke/install.sh`. Neither kit's table declares a row or an
`env_only` entry for it: `lifecycle_kit.rs` and `drift_kit.rs` both carry
`env_only: &[]`.

- **`LIFECYCLE_KIT_SESSIONS_DIR`** is read by `--emit session-id` from the process
  environment, `native/src/emit/session_id.rs`. It is an operator override, and
  lifecycle-kit/SPEC.md refuses it as a row, because a knob file must never set a
  session identity. It is a kit input with no definition site in the payload.
- **`DRIFT_KIT_ITERATION_START`** is computed by the drift driver and exported to
  every plugin it spawns (`native/src/emit/drift_report.rs`). drift-kit/SPEC.md calls
  it never a consumer knob. An adopter writing a plugin reads it, and documents it,
  so it is adopter-facing.
- **`DRIFT_KIT_SMOKE_CUSTOM`** is a name drift-kit's smoke writes into a throwaway
  knob file to prove the open family admits an undeclared name. drift-kit/SPEC.md
  calls it a name no table declares. Declaring it would falsify the property the
  smoke demonstrates.

**Why `env_only` and not a row or a wider scan.** A row would make both names
file-settable. lifecycle-kit refuses that for the first name, and for the second a
file line would be silently clobbered by the driver's export. Widening
`check-docs-cmd` to count `smoke/` passes this tree and leaves the adopter's hard
fail exactly where it was, because the adopter has no `smoke/`. `env_only` does
precisely two things (`native/src/knobs/mod.rs`): it refuses the name in a knob
file at exit 2, and it adds it to `static_names()`. Both are right for both names,
and it mints nothing.

## What changes

### (1) The two environment-resolved names join their kits' `env_only` lists {mechanical}

**Not yet applied.** `native/src/knobs/lifecycle_kit.rs` sets
`env_only: &["LIFECYCLE_KIT_SESSIONS_DIR"]`, and `native/src/knobs/drift_kit.rs`
sets `env_only: &["DRIFT_KIT_ITERATION_START"]`. A crate unit test asserts that
`static_names()` contains both. The oracle is `check-docs-cmd`, run on a scratch copy
of a kit root with its `smoke/` removed, which now finds both defined.

### (2) The `env_only` rule and its refusal cover a handed-through name {mechanical}

**Not yet applied.**

- In gate-sdk/SPEC.md §The knob file, the paragraph **Some names under a kit's prefix
  are set in the environment only.** becomes "**Some names under a kit's prefix
  resolve from the environment only** — set there by a caller, or exported by the kit
  to a process it spawns." It lists this amendment's two names beside gate-sdk's
  four, by kit. It adds one sentence of rule: a name a kit reads or hands through the
  environment takes an `env_only` entry, because `smoke/` is withheld from the
  payload and is therefore no definition site. A name that exists only as smoke
  test data takes none.
- The refusal text in `native/src/knobs/mod.rs` changes from "… — set it in the
  environment" to "… — it is read from the environment, never from a knob file". The
  old wording tells an adopter to export a value the driver computes. The unit test
  pinning the old wording moves with it.

### (3) The two kit SPECs name the site {mechanical}

**Not yet applied.** lifecycle-kit/SPEC.md's sentence refusing the session-id names
as rows gains a clause: `LIFECYCLE_KIT_SESSIONS_DIR` is an `env_only` name, so an
adopter's docs may cite it. drift-kit/SPEC.md's sentence calling
`DRIFT_KIT_ITERATION_START` never a consumer knob gains the same clause.
`DRIFT_KIT_SMOKE_CUSTOM`'s sentence gains nothing, since it already says no table
declares it.

## Producers and consumers

- **The two entries (delta 1).** Producer: the static kit tables. Consumers:
  `knobs::static_names()`, and through it `check-docs-cmd`'s defined set; and the
  knob-file parser's `env_only` refusal. `--emit knob-roster` and the
  `templates/*.knobs` iterate rows only, so neither changes. `check-knob-citation`
  reads SPEC prose, not `env_only`.
- **The refusal wording (delta 2).** Its reader is an adopter whose knob file names
  an environment-only name, and the unit test that pins the wording.
- **Point 6.** The corpus is the three names the entry measured, re-confirmed by
  `find <kit> -type f ! -name "*.md" | xargs grep -l <NAME>`. The values:
  `LIFECYCLE_KIT_SESSIONS_DIR` and `DRIFT_KIT_ITERATION_START` take delta 1.
  `DRIFT_KIT_SMOKE_CUSTOM` needs none, since its only citing document,
  drift-kit/SPEC.md, is withheld from the payload with `smoke/`, so no adopter's
  governed docs can cite it unknowingly.

## Existing sections updated

Rosters from the `find` probe above, `grep -n env_only native/src/knobs/*.rs`, and
`grep -n "set it in the environment" native/src/knobs/mod.rs`.

- `native/src/knobs/lifecycle_kit.rs` and `native/src/knobs/drift_kit.rs` (delta 1).
- gate-sdk/SPEC.md §The knob file and `native/src/knobs/mod.rs` (delta 2).
- lifecycle-kit/SPEC.md and drift-kit/SPEC.md, the two sentences named above
  (delta 3).
<!-- update-target-exempt: generated mirrors, regenerated by their freshness gate's printed command -->
- `docs/gate-sdk/SPEC.md`, `docs/lifecycle-kit/SPEC.md`, `docs/drift-kit/SPEC.md`.

## Retired spellings

- None — the refusal's old wording is a diagnostic string, not a name.

## Definition of Done

- [ ] **Causal completeness.** Every point of canon-kit/SPEC.md §The
      causal-completeness check holds.
- [ ] **Instruction surfaces: instruction only.** Not reached.
- [ ] **Merged with no information lost.** gate-sdk's four names and their grounds
      survive the rewrite.
- [ ] **Amendment deleted.** This file is removed on merge (`ls gate-sdk/SPEC-*.md`).
- [ ] **Entry moved.** `docs-cmd-knob-definition-site-withheld` moves to Done in the
      merge commit, at a stage before the drain stage.
- [ ] **Fails closed.** A knob file naming either new entry is refused at exit 2
      with the new wording, under a unit test.
- [ ] **Removals propagated.** `check-amendment-retired-spelling` runs the block
      above.
- [ ] **Gaps filed.** Any cross-component gap build discovers is resolved that
      session.
