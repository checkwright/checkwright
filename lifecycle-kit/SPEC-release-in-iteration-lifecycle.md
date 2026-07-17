# SPEC amendment: release-in-iteration-lifecycle

Releasing joins the iteration lifecycle: every close dispositions the
iteration at the release boundary — a release executed, or an explicit
no-release stamp — so phase-B work can no longer accrue unreleased with
nothing in the machinery asking. Operator rulings on record (2026-07-17, at
this iteration's scope): tag per qualifying iteration — close derives the
bump, authors the note in-iteration, tags; a both-None iteration tags
nothing by default and stamps the explicit `none` disposition (lead ruling,
derived from docs/install.md §Versioning: an iteration meeting no bump
criterion earns none — a per-iteration patch tag would be new policy, not on
record); a patch stays available on operator judgment for an urgent fix.

## What changes

**1. `templates/skills/close.md` — a new numbered step, Release disposition**
(after the docs-staleness/runtime-artifact steps, before the brevity pass —
the note is a surface-mutating write the brevity pass must follow). Generic
content the template owns:

- Every close dispositions the iteration at the release boundary. Read the
  consumer's release policy (the new slot, below) and either execute its
  release procedure or stamp an explicit no-release line. Silence is not a
  disposition (gap disposition): a close that says nothing about release is
  incomplete.
- The stamp shape, one line into the consumer-named disposition-evidence
  file: `<iteration> release <version|none> — <one-line basis>` (the
  `check-lesson-disposition` contract shape at the iteration's release
  boundary, the same lineage release-sweep's stamp follows). `<version>` is
  the tag applied; `none` states the both-None (or consumer-equivalent)
  outcome.
- Ordering note the template carries: a tag names a commit, so the
  tag-and-host-release half of a consumer's procedure runs after the
  iteration's final commit lands; the note-authoring and stamp halves ride
  the close commits themselves.
- New named slot `*<release-policy: the consumer's release procedure and
  criteria source by citation, the disposition-evidence path, and any
  boundary-only sub-procedures (e.g. a major-only deprecation sweep); or a
  plain "no release process — every iteration stamps none" line for a
  consumer without one.>*`

**2. `bin/enter-stage.sh` — the boundary require-check (the enforcement
half, enforcement-first: the step and the check that catches its skip land
in one unit).** A new config knob:

- `LIFECYCLE_KIT_BOUNDARY_REQUIRE` — array of repo-relative files, default
  empty (an unconfigured consumer sees nothing change — the v0.2.0
  convention). Owned like the other knobs by `lib/stages.sh` defaults +
  §Layout and configuration roster.
- At iteration-boundary entry, before the boundary truncation and after the
  existing Lessons-Learned refusal (same refusal contract, same code path):
  read the closing iteration's name from the queue header; for each
  `BOUNDARY_REQUIRE` member, refuse (writing nothing) unless the file
  carries a data line whose first token is that iteration name. A `—`
  (never-named) closing iteration skips the check — there is nothing to
  disposition. `--simulate` reports the would-be refusal the way it does for
  lessons.
- Fail-closed: a `BOUNDARY_REQUIRE` member that does not exist on disk is a
  refusal naming the path, not a silent pass.

**3. This repo's consumer half (config-via-env, all consumer content):**

- `.claude/commands/close.md` binds the new `release-policy` slot: derive
  the bump per docs/install.md §Versioning off the note's two sections;
  author the dated `docs/posts/` note per RELEASING.md (grammar owned by
  docs/install.md §The upgrade contract); qualifying iteration → RELEASING.md's
  tag / GitHub Release / badge steps, major → release-sweep first;
  both-None → stamp `none`. Disposition evidence:
  `.workflow/release-disposition.txt` (committed, like the other
  `.workflow/` projections).
- `scripts/lifecycle-config.sh`: add `.workflow/release-disposition.txt` to
  both `LIFECYCLE_KIT_BOUNDARY_REQUIRE` (the new knob) and
  `LIFECYCLE_KIT_BOUNDARY_TRUNCATE` (the file holds the current iteration's
  disposition; git history is the archive). Require-check runs before
  truncation, so the line just checked is consumed by the same boundary
  that verified it.
- **RELEASING.md reordered** (operator ruling: the reorder is owned
  explicitly, not implied): the runbook reframes from an operator-invoked
  procedure to the close-stage release procedure invoked by the close
  skill's release-disposition step at every iteration close. Step order
  becomes: (1) author the note in-iteration; (2) derive the bump off the
  note per §Versioning — both-None → stamp `none`, stop; (3) major only:
  release-sweep before the tag; (4) tag the iteration's final commit;
  (5) GitHub Release pointing at the post; (6) badge verify. The
  no-release disposition line joins the runbook so the both-None ruling
  has its durable governed home here.

## Producers and consumers

- **`release-policy` slot** — producer: the close template (this
  amendment); consumer: every close binding shim (`check-skill-binding`
  holds shim↔template slot parity, so this repo's
  `.claude/commands/close.md` must bind it in the same change — the gate is
  the parity's named reader).
- **Disposition stamp line** — producer: the close session executing the
  step (enabling config: the slot binding + this repo's
  `lifecycle-config.sh` entry, both landed by this unit, so the producer is
  deployed, not test-only); consumers: `enter-stage.sh`'s boundary
  require-check reads the iteration token mechanically (the named reader
  that makes the stamp enforced, not decorative); the `<version|none>` and
  basis fields are operator evidence riding the close commit, the honest
  release-sweep precedent (lifecycle-kit/SPEC.md §templates/skills/ states
  the same limit for its stamp file), read at audit and by the next
  release's author.
- **`LIFECYCLE_KIT_BOUNDARY_REQUIRE`** — producer: consumer config
  (`scripts/lifecycle-config.sh` sets it here); consumer: `enter-stage.sh`'s
  boundary branch; default empty keeps every unconfigured consumer's
  behavior byte-identical.
- **`.workflow/release-disposition.txt`** — consumer-named path (kit ships
  no literal); its `BOUNDARY_TRUNCATE` membership feeds the derived
  iteration-scoped supersede set (`lib/stages.sh`), so `.gitattributes`
  gains its `merge=iteration-scoped` line and `check-merge-attrs` is the
  freshness reader — the build regenerates the attributes block or the gate
  reds.

## Existing sections updated

- lifecycle-kit/SPEC.md §templates/skills/ — the close-template description
  gains the release-disposition step and slot; the release-sweep paragraph
  stays major-only but its invocation context becomes "from close's
  release-disposition step when the derived bump is a major" rather than
  free-floating.
- lifecycle-kit/SPEC.md §bin/enter-stage.sh and §Layout and configuration —
  the require-check contract and the knob row.
- lifecycle-kit/SPEC.md §check-stage-entry / gate-tests — the new refusal is
  enter-stage's (tool-side), mirroring the lessons refusal; cover the
  require-check scenarios (present, missing, unnamed iteration, missing file
  fail-closed). Align note (2026-07-17): enter-stage.sh's boundary refusals
  (lessons, truncation) have **no dedicated unit test** today — only
  `smoke/install.sh` exercises the tool end-to-end — so this is a new
  `enter-stage` test harness (or an extension of the smoke), not an edit to an
  existing one.
- RELEASING.md — the reorder above (this repo's doc; the note grammar and
  §Versioning criteria stay owned by docs/install.md, cited never restated).
- CLAUDE.md §Housekeeping's RELEASING.md sentence — "resident only at a
  release" becomes "resident only at close's release step" (coordinate with
  the claude-md-housekeeping-residency unit, which rewrites the same
  section; that unit owns the section's final shape, this one owns the fact).
- `check-release-bump` (this repo's scripts/) — unchanged; it keeps gating
  the derivable floor and now simply fires per-iteration instead of
  per-rare-release.

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
