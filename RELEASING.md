# Releasing Checkwright

The repeatable release procedure — the **close stage's release-disposition
step** invoked at *every* iteration close (lifecycle-kit/templates/skills/close.md;
this repo binds it in `.claude/commands/close.md`), not a separately
operator-invoked ritual. Checkwright carries one semver line applied as git tags,
the kits moving in lockstep — the versioning model is
[docs/install.md](docs/install.md) §Versioning, and the two-phase contract a
release serves is that page's §The upgrade contract. The release-note post is the
note's single home and the GitHub Release points at it.

Like [CONTRIBUTING.md](CONTRIBUTING.md), this runbook is governed repo-meta:
tracked, pinned in `scripts/core-files.list`, and in the spec manifest so its
links and commands resolve under the doc gates.

## The procedure

Every iteration close dispositions the release boundary and records the outcome
as one line in `.workflow/release-disposition.txt` (the boundary-required
disposition evidence read by the next iteration's scope entry,
lifecycle-kit/SPEC.md §bin/enter-stage.sh): `<iteration> release <version|none> —
<one-line basis>`.

1. **Author the release-note post — in-iteration.** Add a dated `docs/posts/`
   entry carrying a `release: vX.Y.Z` front-matter key and the two sections the
   upgrade contract names — tightened gates and renamed knobs. The parseable
   grammar (front-matter key, section names, bullet lead tokens) is owned by
   [docs/install.md](docs/install.md) §The upgrade contract; "none" is a valid
   section body and is stated, never omitted. The note joins the nav by its
   `release:` key with no further step — the [Releases page](docs/releases.md)
   names that key in `nav_children_key`, so the note renders as a derived nav
   child (and the render-time releases index lists it likewise); no
   `scripts/docs-offnav.list` entry, no allowlist growth per release.
   Launch-adjacent copy uses the reserved phrasing, "the verification layer
   under agent orchestration".

2. **Derive the bump off the note.** Choose the bump by
   [docs/install.md](docs/install.md) §Versioning's criteria (the derivable floor
   is gated by `check-release-bump`), read off the note's two sections. An
   iteration meeting no bump criterion earns **none**: stamp `<iteration> release
   none — <basis>` into the disposition evidence and stop — no tag, no GitHub
   Release. A patch stays available on operator judgment for an urgent fix.
   Otherwise continue with the derived `vX.Y.Z`.

3. **Major only: sweep the deprecation markers.** When the bump is a major, run
   the release-sweep skill *before* the tag — the deprecation disposition walk at
   the release boundary, its contract lifecycle-kit/SPEC.md §templates/skills/.
   Every marker on the roster earns a stamped disposition in
   `.workflow/release-sweep-evidence.txt` before the tag; an empty roster is a
   stated "none". No marker rides into the next major undispositioned.

4. **Tag the iteration's final commit.** Tag `vX.Y.Z` on the iteration's final
   commit and push it — `git tag -a vX.Y.Z` on the commit, then push the tag to
   the origin. Push mechanics for a keyless agent sandbox live in the operator's
   local ops runbook, outside the tree. Stamp `<iteration> release vX.Y.Z —
   <basis>` into the disposition evidence.

5. **Create the GitHub Release.** Its body points at the post's
   `https://checkwright.dev/` URL — the post is the note's single home, the
   Release a pointer to it, never a second copy of the note.

6. **Verify the version badge.** Confirm the README release-version badge
   resolves the new tag. It is sourced from the GitHub tag list, so each release
   updates it with no edit — this step is a verification, not a write.
