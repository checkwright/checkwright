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
**First-hand demand, recorded here because it otherwise lives only in agent
transcripts:** during this iteration the by-hand wipe destroyed two scratch
members that needed to survive the boundary — the live lead's `session-role`
marker and the scope session's resume journal — because a hand-run `find …
-delete` carries no keep-list. The failure this unit exists to prevent
reproduced twice inside the iteration that fixes it.

New knob **`LIFECYCLE_KIT_BOUNDARY_PRESERVE`** — a bash array of scratch-dir
member **basenames** to keep across the wipe; **default empty**. At the
iteration boundary, after the truncate loop, enter-stage deletes every member
of `GATE_SDK_TMP_DIR` whose basename is not listed, then names the wiped set in
its boundary report exactly as it already names the truncated set. The wipe is
**boundary-only** (the `first == 1` path); non-boundary stage entries append and
touch no scratch. The wipe is unconditional over the scratch dir because scratch
is disposable *by definition* (CLAUDE.md §Housekeeping — persistent trends live
in `.metric/`, never in scratch). It carries exactly one **kit-level exemption**:
`.gitkeep`, which is not scratch content but the directory's own tracked
scaffolding. The kit must not delete a file the consumer tracks — doing so
removes a tracked file at the boundary and dirties the tree at the moment the
scope session commits the reset. (The exemption is *not* about the directory
surviving: `bin/enter-stage.sh:58` already `mkdir -p`s the scratch dir, and the
wipe removes members, not the dir. Recorded with that reason instead, a later
reader would retire the exemption as redundant.) **Align audit — the exemption is
generic-consumer mechanism, not a fact about this tree.** This repo gitignores the
scratch dir wholesale (`.gitignore:4`) and keeps no `.gitkeep` in it (the only
tracked one is a context-kit fixture under a different directory), so the
tracked-file hazard is the one a consumer that *does* commit its scratch dir's
scaffolding faces. Checking the "tracked scaffolding" phrase against this checkout
finds nothing and would invite retiring the invariant — the same retire-it failure
the parenthetical above guards against, reached by the other road. context-kit's session sweep over
this identical directory already exempts `.gitkeep` by name, so the two sweeps
agree on what is not scratch; match its form — a basename test (`! -name
.gitkeep`) applied at every depth the wipe reaches, consistent with `PRESERVE`
members being basenames. Beyond that one invariant,
`LIFECYCLE_KIT_BOUNDARY_PRESERVE` is the single **consumer** control and stays
default-empty, so an unset-knob consumer gets a clean wipe of an
already-disposable surface.

**The exemption and the keep-list are different tiers.** The `.gitkeep`
exemption is a **kit invariant** the consumer cannot unset; `PRESERVE` is
**configuration** layered on top of it. The "single control" wording above
records one decision — the refusal of a *second knob* (a wipe enable/disable
toggle beside `PRESERVE`) — and a hardcoded exemption configures nothing, so it
is not a control and leaves that decision standing; what it corrects is an
over-broad literal reading, never a deliberate assertion that `.gitkeep` be
deleted. **Ruled out — collapsing the tiers by shipping the exemption as
`PRESERVE`'s default.** A defaulted bash array is *replaced*, not merged, when a
consumer assigns it, so protection would decrease as configuration increases and
any consumer setting `PRESERVE` for its own reasons would silently lose the
exemption — this consumer, which sets `(session-role)`, is exactly that case.
**Ruled out — a git-aware "skip any tracked file" rule.** It makes filesystem
behavior git-dependent for one case, and it would preserve *any* tracked file
parked in scratch, re-opening the accumulation this unit exists to close.

**Truncate vs wipe are distinct operations, not one knob.** `TRUNCATE` rewrites
a *tracked* file down to its `# contract:` header (the file must survive with an
empty body); the wipe *deletes* untracked scratch members outright. They share
the boundary trigger and the report line, nothing else — `PRESERVE` is a keep-list
for the delete operation, never a truncate target.

**The scratch dir already has a reclaimer; this is the second, and they do not
overlap.** context-kit's session-context hook sweeps the same
`${GATE_SDK_TMP_DIR:-.tmp}` at **every session start**, age-guarded (`-mmin
+1440`) precisely so a concurrent same-checkout session's in-flight scratch
survives (context-kit/SPEC.md §The session-context hook, step 6). The boundary
wipe is deliberately **not** age-guarded: it fires once, at the iteration
transition, where the only scratch a consumer means to carry across is by
definition named in `PRESERVE` — an age guard there would leave the previous
iteration's fresh residue behind, which is the whole thing being reclaimed. Two
triggers, two postures, one directory; neither mechanism reads the other and no
context-kit surface changes. The other cross-component scratch residents are
already consistent with a wipe: drift-kit's `DRIFT_KIT_TMP_DIR` members are
"regenerated on every run, so a scratch wipe is harmless" (drift-kit/SPEC.md
§Layout and configuration), its persistent trends live in `DRIFT_KIT_METRIC_DIR`
by retention contract, and the knowledge-friction log is a `.workflow/` member,
not scratch.

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

- **New state: the boundary-wipe set** (scratch-dir members minus the `.gitkeep`
  kit invariant, minus `PRESERVE`).
  - *Producer:* `bin/enter-stage.sh`, the `first == 1` (iteration-boundary) path,
    after the truncate loop (currently :177-183) and after its own temp-file
    cleanup (:190), so its `enter-stage.*.$$` temporaries are already gone and are
    never candidates. Enabling config: `LIFECYCLE_KIT_BOUNDARY_PRESERVE`, read via
    `lib/stages.sh` (the loader that already defaults the other boundary knobs); a
    fall-open empty default when the consumer sets nothing. The scratch dir is
    read from `GATE_SDK_TMP_DIR` (`:57`, already resolved for the temp files) — no
    new dir knob. *Align audit — delete mechanics:* mirror context-kit's form
    (`-mindepth 1`, `-depth`, stderr suppressed). `-depth` so a directory is
    removed after its contents; the suppression because a `PRESERVE` basename kept
    *inside* a doomed subdirectory makes that subdirectory's own delete fail. That
    failure is noise, never an abort — `bin/enter-stage.sh` runs `set -uo pipefail`
    with **no `-e`**, verified at `:3`.
  - *Consumer:* the delete itself (the boundary actor is its own consumer — this
    is a filesystem side effect, not a message), and the **boundary report**
    (`:199-201`), which gains a `note:` line naming the wiped members, mirroring
    the existing truncate `note:`. The report's reader is the operator/agent
    reading enter-stage's stdout.
- **New field: each `PRESERVE` member (a basename).**
  - *Named reader:* the wipe loop, at the boundary transition, tests each
    scratch-dir member's basename against the `.gitkeep` invariant and then the
    `PRESERVE` set to decide keep-vs-delete.
    A member listed but absent from the scratch dir is a harmless no-op (nothing to
    keep); a member present and matching neither is deleted. No other transition
    reads it. The invariant is not a `PRESERVE` member and is unaffected by what
    the consumer assigns — the tier split above.

## Existing sections updated

- **lifecycle-kit/SPEC.md §bin/enter-stage.sh** — the boundary-reset contract
  gains the wipe step, its `.gitkeep` invariant, and its report line, placed after
  the truncate description; state that truncate (tracked-file → header) and wipe
  (untracked scratch delete) are distinct, sharing only trigger and report.
- **lifecycle-kit/SPEC.md §Layout and configuration** — the boundary-knob roster
  gains `LIFECYCLE_KIT_BOUNDARY_PRESERVE` (array, default empty, keep-list of
  scratch-dir basenames), noted as boundary-only and paired with the scratch dir
  it reads (`GATE_SDK_TMP_DIR`). The entry states the **tier split** explicitly:
  the `.gitkeep` exemption is a kit invariant this knob cannot unset, `PRESERVE`
  is the consumer keep-list layered on top — a reader who takes the knob for the
  wipe's only keep rule would reintroduce the tracked-file deletion.
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
