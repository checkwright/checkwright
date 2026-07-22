# SPEC amendment: boundary-scratch-preserve

<!-- Delta for the `boundary-scratch-wipe-unowned` unit of the
     permission-posture-reconciliation iteration. Merges into
     lifecycle-kit/SPEC.md on completion; delete this file then. -->

## What changes

The iteration-boundary reset gains a **scratch-wipe step**. Today
`bin/enter-stage.sh`'s first-stage (boundary) path resets `WORKFLOW-STATE`,
renames the queue header, and *truncates-to-header* the kit-owned
lesson-evidence file plus every `LIFECYCLE_KIT_BOUNDARY_TRUNCATE` member
(enter-stage.sh:174-183). It does **not** clear the disposable scratch that
accumulates in the scratch dir (`GATE_SDK_TMP_DIR`, default `.tmp/`) across an
iteration — validate logs, probe scripts, resume journals. CLAUDE.md
§Housekeeping already asserts `.tmp/` is "wiped at the scope boundary", but no
mechanism performs it, so every scope session re-authors a destructive
`find … -delete` by hand and pays a permission prompt for it (the unit's cost).

New knob **`LIFECYCLE_KIT_BOUNDARY_PRESERVE`** — a bash array of scratch-dir
member **basenames** to keep across the wipe; **default empty**. At the
iteration boundary, after the truncate loop, enter-stage deletes every member
of `GATE_SDK_TMP_DIR` whose basename is not listed, then names the wiped set in
its boundary report exactly as it already names the truncated set. The wipe is
**boundary-only** (the `first == 1` path); non-boundary stage entries append and
touch no scratch. The wipe is unconditional over the scratch dir because scratch
is disposable *by definition* (CLAUDE.md §Housekeeping — persistent trends live
in `.metric/`, never in scratch); `PRESERVE` is the single control, so an
unset-knob consumer gets a clean wipe of an already-disposable surface.

**Truncate vs wipe are distinct operations, not one knob.** `TRUNCATE` rewrites
a *tracked* file down to its `# contract:` header (the file must survive with an
empty body); the wipe *deletes* untracked scratch members outright. They share
the boundary trigger and the report line, nothing else — `PRESERVE` is a keep-list
for the delete operation, never a truncate target.

**Seam.** The wipe mechanism and the empty-default knob are generic
lifecycle-kit mechanism. The one value this consumer sets — `session-role` — is
**consumer config**, landing in `scripts/lifecycle-config.sh` beside the existing
`LIFECYCLE_KIT_BOUNDARY_TRUNCATE=(…)` line, exactly the established pattern. The
*reason* that value is preserved (it is **context-kit's** session marker, whose
lifetime is the live lead session's, not the iteration's —
context-kit/SPEC.md §The session-context hook) is cited by the consumer config,
not owned by the kit. No context-kit surface changes: the marker stays where its
hook already reads it (`.tmp/session-role`).

**Ruled out — option (b), dissolve the exception by relocating the marker.** The
unit filed an alternative: have the kit wipe blanket and move `session-role` out
of the scratch dir entirely, so nothing needs preserving. Rejected. It requires a
**context-kit** change (relocate the marker, update its hook and SPEC); it risks
breaking the *live* session-role suppression mid-session, the precise hazard the
scope evidence-reset binding warns of; and it does not even avoid a governed name
— it trades the `PRESERVE` knob for a new marker-location convention while
touching a second kit. Option (a) — this amendment — adds one knob following an
existing config pattern, touches one kit, and leaves the working marker alone.

## Producers and consumers

- **New state: the boundary-wipe set** (scratch-dir members minus `PRESERVE`).
  - *Producer:* `bin/enter-stage.sh`, the `first == 1` (iteration-boundary) path,
    after the truncate loop (currently :177-183) and after its own temp-file
    cleanup (:190), so its `enter-stage.*.$$` temporaries are already gone and are
    never candidates. Enabling config: `LIFECYCLE_KIT_BOUNDARY_PRESERVE`, read via
    `lib/stages.sh` (the loader that already defaults the other boundary knobs); a
    fall-open empty default when the consumer sets nothing. The scratch dir is
    read from `GATE_SDK_TMP_DIR` (`:57`, already resolved for the temp files) — no
    new dir knob.
  - *Consumer:* the delete itself (the boundary actor is its own consumer — this
    is a filesystem side effect, not a message), and the **boundary report**
    (`:199-201`), which gains a `note:` line naming the wiped members, mirroring
    the existing truncate `note:`. The report's reader is the operator/agent
    reading enter-stage's stdout.
- **New field: each `PRESERVE` member (a basename).**
  - *Named reader:* the wipe loop, at the boundary transition, tests each
    scratch-dir member's basename against the `PRESERVE` set to decide keep-vs-delete.
    A member listed but absent from the scratch dir is a harmless no-op (nothing to
    keep); a member present and unlisted is deleted. No other transition reads it.

## Existing sections updated

- **lifecycle-kit/SPEC.md §bin/enter-stage.sh** — the boundary-reset contract
  gains the wipe step and its report line, placed after the truncate description;
  state that truncate (tracked-file → header) and wipe (untracked scratch delete)
  are distinct, sharing only trigger and report.
- **lifecycle-kit/SPEC.md §Layout and configuration** — the boundary-knob roster
  gains `LIFECYCLE_KIT_BOUNDARY_PRESERVE` (array, default empty, keep-list of
  scratch-dir basenames), noted as boundary-only and paired with the scratch dir
  it reads (`GATE_SDK_TMP_DIR`).
- **Consumer surfaces (this repo, not kit):** `scripts/lifecycle-config.sh` sets
  `LIFECYCLE_KIT_BOUNDARY_PRESERVE=(session-role)` with the context-kit citation;
  CLAUDE.md §Housekeeping's "wiped at the scope boundary" clause is now *mechanized*
  and points at the mechanism rather than implying a by-hand wipe. These land at
  build, not merge, but are named here so the causal chain is complete.

## Definition of Done

- [ ] **Causal completeness** — the wipe has a reachable producer (boundary path,
      post-truncate, post-temp-cleanup) and its consumer/report; the `PRESERVE`
      member field has a named reader (the keep-vs-delete test).
- [ ] **Merged with no information lost** — the wipe step integrated into
      §bin/enter-stage.sh and the knob into §Layout and configuration; the merged
      SPEC reads coherently for a reader who never saw this file.
- [ ] **Amendment deleted** — this file removed on merge (`ls lifecycle-kit/SPEC-*.md`).
- [ ] **Removals propagated** — no name retired by this change.
- [ ] **Gaps filed** — any cross-component gap found during build resolved that
      session, not deferred.
