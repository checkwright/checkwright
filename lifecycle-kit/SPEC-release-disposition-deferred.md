# SPEC amendment: release-disposition-deferred-value

Adds a third release-disposition value for a release the criteria earn but an
operator ruling holds back, and defines the mechanism that carries the unconsumed
criteria forward — the half the queue entry named as the real design.

## What changes

**The grammar gains a third value.** The release-disposition line
(§The release-disposition step) becomes:

```
<iteration> release <version|none|deferred:<version>> — <one-line basis>
```

`deferred:<version>` means *the criteria for `<version>` are met and the release
was held back*. It is distinct from `none` ("nothing to release") — the
distinction `none` lost when `verdict-reader-honesty` overloaded it, forcing any
reader to parse basis prose to tell the two apart.

**Why `deferred:<version>` and not a bare `deferred`.** The thing that must
survive is the *earned bump level*; a bare token drops it and the next release
re-derives which floor it inherits, which is the recurrence cost this unit was
filed on.

**How the producer derives `<version>`.** The field is the version the criteria
*would have shipped as* had the release not been held: the bump the note's three
upgrade-contract sections floor, applied over the newest already-released note.
It is the version that was earned and withheld, never the next version the
project happens to reach. For this repo's live carrier, `verdict-reader-honesty`
met minor criteria over a newest release of `v0.9.0`, so its line stamps
`deferred:v0.10.0`. Stating the rule here is what keeps the field mechanically
derivable rather than an operator's guess — without it the discharge comparison
has no defined scale.

**Why the criteria stay in prose and are not structured fields.** The three
upgrade-contract sections (Tightened gates / Renamed knobs / Behavior changes)
are the release note's, and docs/install.md §The upgrade contract owns them. A
structured criteria list on the disposition line would be a second copy of a
surface that already exists — the de-literalization rule applied to this line:
it carries the *level*, the note owns the *criteria*.

**Outstanding-deferral is derived, never tracked.** A `deferred:<version>` line
is **outstanding** until a later line dispositions a release at or above
`<version>`; that later line **discharges** it. Nothing records discharge
separately — the release actually happening is the discharge, so there is no
second state to drift. This is what keeps the widened gate low-false-positive: a
deferral cannot linger past the release that consumes it, and a deferral that
genuinely has not been consumed *should* keep firing, which is the point.

**The carry-forward reader reads the union of committed history and the live
file.** This is the load-bearing finding, and it was verified rather than
assumed: `.workflow/release-disposition.txt` is a
`LIFECYCLE_KIT_BOUNDARY_TRUNCATE` member, and it is **already header-only** —
the `verdict-reader-honesty` line carrying this repo's unconsumed minor criteria
survives only in commit `6c53737`. A reader of the live file alone therefore
sees nothing and would gate nothing.

Any consumer-side gate over this line derives its disposition set the same way
`drift-kit/SPEC-stage-economics-durability.md` derives the stamp set and
`drift-kit/bin/trajectory.sh` already ships: **history ∪ live — not
replacement, not fallback.** The two amendments share one technique under one
vocabulary, deliberately; a reader who has understood either has understood
both. History-only is the *replacement* arm the stamp amendment rules out, and
it fails harder here than there: `check-release-bump` is `tier=precommit`, so
the run that matters most is the pre-commit of the very close commit that
writes the `deferred:` line — at that instant the line is live and not yet
committed, and a history-only reader is blind to precisely the disposition it
was widened to enforce. The live arm covers that uncommitted tail; the history
arm covers everything truncation has taken. Union costs nothing, because a line
present in both arms is the same line and dedups on the iteration name.

Truncation-immunity is a property of the *reader*, and every reader of a
truncated evidence file needs it.

**The kit wires no gate.** Consistent with the release-sweep stamp file
(§The release-disposition step), the disposition file is operator evidence riding
the release commit; the kit defines the value and the outstanding/discharged
derivation, and a consumer may gate it. The split is the provenance seam: the
*grammar* is generic lifecycle mechanism, the *bump criteria* being carried are
consumer release policy.

**Consumer side (this repo).** `scripts/check-release-bump.sh` — which today
couples `docs/posts/*.md` and `docs/install.md` only and never reads the
disposition file at all — widens to derive outstanding deferrals from the
disposition file's history and to floor the newest note's bump against the
highest outstanding `deferred:<version>`. Its manifest gains the disposition
path, becoming:

```
# graph: couples=docs/posts/*.md,docs/install.md,.workflow/release-disposition.txt dir=one valve=none tier=precommit
```

Landing that regenerates the pre-commit hook and the graph artifact per
CLAUDE.md. `dir=one` is unchanged and correct: the note derives its floor from
the disposition record, never the reverse.

**The deferred floor derives before the two-note early return.** The gate today
exits clean when fewer than two notes carry a `release:` key
(`scripts/check-release-bump.sh:26-29`), because its existing floor is a
*comparison against a predecessor note*. The deferred floor has no such
dependency — it is carried by the disposition record, not by a predecessor — so
a single-note tree with an outstanding deferral must still red. The derivation
therefore runs ahead of that return, not inside the two-note arm. This repo has
nine keyed notes so the arm is unreachable here today; it is specified because a
fresh consumer adopting the kit meets it on note one, which is exactly when a
carried deferral is easiest to lose.

## Producers and consumers

**New state: the `deferred:<version>` disposition line.**

- *Producer* — the `close` template's release-disposition step
  (`templates/skills/close.md`), under the consumer's `release-policy` slot,
  which already carries the disposition-evidence path and the criteria by
  citation. The enabling config is already emitted in this repo:
  `scripts/lifecycle-config.sh:4-5` wires `.workflow/release-disposition.txt`
  into both `LIFECYCLE_KIT_BOUNDARY_TRUNCATE` and `LIFECYCLE_KIT_BOUNDARY_REQUIRE`
  — so the producer needs no new configuration to fire, and the close step that
  writes the line is the same step that writes `none` today.
- *Consumers*, all three named:
  1. `bin/enter-stage.sh`'s boundary require-check
     (`LIFECYCLE_KIT_BOUNDARY_REQUIRE`, §bin/enter-stage.sh). **Verified
     unchanged**: it tests only that a data line's *first* token is the closing
     iteration's name (`bin/enter-stage.sh:144-146`) and never parses the value
     field, so a third value passes it without modification. Named here so the
     merge does not widen a check that needs no widening.
  2. `scripts/check-release-bump.sh` — the new mechanical reader, consuming the
     `<version>` field at the transition where it derives the newest note's floor
     over its predecessor.
  3. The operator at the next qualifying release, reading the basis prose for
     which criteria are outstanding.

**Field readers.** The line has two fields and both have a named reader at a
named transition: `<version>` is read by `check-release-bump`'s floor comparison
(at the patch-only-bump test, `scripts/check-release-bump.sh:53-65`); the basis
prose is read by the operator when writing the next note's three sections. No
field is added without a reader.

**No new knob.** The disposition path is already `release-policy`-named on the
kit side and already wired on the consumer side; nothing new needs configuring.

## Existing sections updated

- **§The release-disposition step.** The grammar line
  `<iteration> release <version|none> — <basis>` gains the third value, with the
  `none`-vs-`deferred` distinction stated in one line, the derived
  outstanding/discharged rule, and the kit-wires-no-gate posture made explicit
  for the new value.
- **§bin/enter-stage.sh.** The `LIFECYCLE_KIT_BOUNDARY_REQUIRE` paragraph gains
  one clause recording that the check is value-agnostic by construction (first
  token only), so a future value addition does not re-derive whether it needs
  widening.
- **`.workflow/release-disposition.txt` header.** The file's contract header
  restates the grammar (`:2-3`); it is updated in the same unit, or it ships a
  grammar the file itself contradicts.
- **`RELEASING.md`.** The close-stage release procedure gains the deferred arm:
  when the criteria are met and the operator holds the release, stamp
  `deferred:<version>` and carry the outstanding criteria into the next
  qualifying note's three sections.
- **`docs/install.md` §Versioning.** The derivable bump floor gains its second
  input — a note inherits any outstanding deferred version's floor, not only the
  floor its own sections derive.
- **`scripts/check-release-bump.sh` header.** Its couples manifest (above) and
  its `# spec:` pointer both widen to name the disposition surface.

## Seam

Split, and the split is the design. Kit side, generic: the third value, its
grammar, and the outstanding/discharged derivation — no version number, release
cadence, or bump criterion is a kit literal. Consumer side, private-to-this-repo:
which criteria floor a bump and what this repo's release procedure is, which stay
in `docs/install.md` and `RELEASING.md` where they already live. The kit ships no
list of criteria.

## Definition of Done

- [ ] **Causal completeness** — every new state/event/interface has a named,
      reachable producer and a named consumer; every new field has a named
      reader at a named transition.
- [ ] **Merged with no information lost** — each addition integrated into its
      proper canonical-spec section (not appended); the merged spec reads as one
      coherent document a reader who never saw the amendment can use alone.
- [ ] **Amendment deleted** — this file removed on merge; none remain for the
      component (`ls lifecycle-kit/SPEC-*.md`).
- [ ] **Removals propagated** — grepped every spec for names this change
      retired; nothing dangles.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed as
      debt tasks (a build-time causal gap is resolved that session, not
      deferred).
- [ ] **The live carrier is recovered** — this repo's outstanding
      `verdict-reader-honesty` minor criteria, currently surviving only in commit
      `6c53737`, are re-stamped as a `deferred:` line and the widened gate reds a
      patch-only note against them. The unit is not done while the criteria it
      exists to carry remain carried by memory alone.
