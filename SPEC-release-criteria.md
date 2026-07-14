# SPEC amendment: release-criteria

## What changes

No doc states what earns a major, minor, or patch — docs/install.md
§Versioning defines the line (one semver line, lockstep kits, tags) but no
bump criteria, and the only major semantics on record are implied by the
deprecation machinery (release-sweep's disposition walk, the knob-rename
deprecation owing). This amendment lands the criteria in
**docs/install.md §Versioning** (the page RELEASING.md already cites for
the versioning model; RELEASING.md itself gains no second copy).

**The criteria — derived from the note, not maintained beside it.** The
release note's two fixed sections (Tightened gates, Renamed knobs) already
declare everything phase B must reconcile, so the bump floor derives from
the note (derivation-first):

- **Patch** — both note sections are "None": a phase-A-only sync (fixes
  and docs that tighten nothing a consumer must reconcile).
- **Minor** — either section is non-empty: the release carries phase-B
  work — a new or stricter gate, or a knob rename riding its deprecation
  path.
- **Major** — a decommission: a release that *removes* a deprecated
  surface (a release-sweep disposition executed as decommission), or any
  change the two-phase upgrade contract cannot reconcile from the note
  alone. Majors are where the deprecation promises come due — the
  release-sweep constraint ("no marker rides into the next major
  undispositioned") binds here.
- **Pre-1.0 qualifier** — while the line is 0.x, breaking changes may ride
  minors (the semver 0.x convention), each still declared in the note;
  `v1.0.0` is the first stability promise and is cut deliberately, never
  earned mechanically.

**The oracle** (enforcement-first — criteria with no gate are prose-only):
a consumer gate `scripts/check-release-bump.sh` parses the `release:`
front-matter keys under `docs/posts/`, orders the versions, and asserts
the derivable floor on the newest note: non-empty Tightened-gates or
Renamed-knobs sections with a patch-only bump over the predecessor note is
red. The major criteria stay judgment (a decommission is a semantic fact
the gate cannot read); the gate holds only the floor. It lands in this
repo's `scripts/` (the producer repo is the only repo that authors release
notes — consumers never do, so this is consumer-gate altitude, no kit
change), with the standard fixture pair under `scripts/gate-tests/`,
registered in `gates.list`, tier `precommit`.

## Producers and consumers

- **The criteria prose** — producer: this amendment, merged into
  docs/install.md §Versioning; consumers: the operator cutting a tag
  (RELEASING.md step 3 reads §Versioning by its existing citation) and any
  consumer reading the versioning model.
- **The gate** — producer: the pre-commit hook and `run-gates.sh` via the
  `gates.list` registration; consumer: the committing session authoring a
  release note. Reads only existing surfaces (`docs/posts/` front matter
  and section bodies whose grammar docs/install.md §The upgrade contract
  owns); adds no new field, so no new reader obligations.

## Existing sections updated

- docs/install.md §Versioning — gains the criteria block.
- RELEASING.md — step 3 gains the half-line "choose the bump by
  §Versioning's criteria" (a citation, not a restatement).
- Regeneration ride-alongs on landing: pre-commit hook + graph artifact
  (new gate manifest), enforcement map + value rollup (new `tier=`
  registration). docs/install.md is a live docs page, not a mirrored kit
  SPEC — no mirror regen.

## Definition of Done

- [ ] **Causal completeness** — every new state/event/interface has a named,
      reachable producer and a named consumer; every new field has a named
      reader at a named transition.
- [ ] **Merged with no information lost** — each addition integrated into its
      proper canonical-spec section (not appended); the merged spec reads as one
      coherent document a reader who never saw the amendment can use alone.
- [ ] **Amendment deleted** — this file removed on merge; none remain at the
      repo root (`ls SPEC-*.md`).
- [ ] **Removals propagated** — grepped every spec for names this change
      retired; nothing dangles.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed as
      debt tasks (a build-time causal gap is resolved that session, not
      deferred).
