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
   entry carrying a `release: vX.Y.Z` front-matter key and the three sections the
   upgrade contract names — tightened gates, renamed knobs, and behavior changes.
   The parseable grammar (front-matter key, section names, bullet lead tokens) is
   owned by [docs/install.md](docs/install.md) §The upgrade contract; "none" is a
   valid section body and is stated, never omitted. The note joins the nav by its
   `release:` key with no further step — the [Releases page](docs/releases.md)
   names that key in `nav_children_key`, so the note renders as a derived nav
   child (and the render-time releases index lists it likewise); no
   `scripts/docs-offnav.list` entry, no allowlist growth per release.

   **Author the note's fixed chrome from this skeleton, never by copying a prior
   post** — this runbook is the chrome's single source (the verbatim text below
   was lifted once from the current posts to seed it, not copied per release):

   - **Opener (fixed, verbatim)** — the reserved framing, then a per-release
     summary slot: *"Checkwright is the verification layer under agent
     orchestration, and this release {one- or two-sentence summary}."*
   - **The three variable sections** — Tightened gates, Renamed knobs, and
     Behavior changes, authored to [docs/install.md](docs/install.md) §The
     upgrade contract's grammar (a knob *removal* is expressed `old → ∅` under
     Renamed knobs); that pointer owns their grammar, this skeleton does not
     restate it.
   - **Upgrading — sync/regen slot** — {the wholesale kit sync at `vX.Y.Z`, the
     generated artifacts to regenerate, then the full battery}.
   - **Upgrading — allowed-red slot (two-way)** — state either "**No allowed
     reds.**" when Tightened gates is empty, or "**The allowed red[s].**" naming
     each red and the regen or step that clears it. The set itself — the
     Tightened-gates lead tokens — is owned by [docs/install.md](docs/install.md)
     §The upgrade contract's allowed-red-set grammar; cite it, never restate it.
   - **Closing tail (fixed, verbatim)** — "The behavior changes above are
     declared for reading, not a mechanical scan. If a gate reds that this note
     does not name, the upgrade smoke was supposed to catch it first —
     [open an issue](https://github.com/checkwright/checkwright/issues), because
     that is a defect in the release rather than work for you."

2. **Derive the bump off the note.** Choose the bump by
   [docs/install.md](docs/install.md) §Versioning's criteria (the derivable floor
   is gated by `check-release-bump`), read off the note's three sections. An
   iteration meeting no bump criterion earns **none**: stamp `<iteration> release
   none — <basis>` into the disposition evidence and stop — no tag, no GitHub
   Release. A patch stays available on operator judgment for an urgent fix.
   An iteration that **meets** a bump criterion but whose release the operator
   holds back is neither of those: stamp `<iteration> release deferred:vX.Y.Z —
   <basis>` — the version the criteria would have shipped as, derived over the
   newest already-released note — and stop, again with no tag and no Release. The
   deferral stays outstanding until a later line releases at or above it, and the
   outstanding criteria are carried into the next qualifying note's three
   sections; `check-release-bump` floors that note against it. Never overload
   `none` for this — `none` means nothing was earned.
   Otherwise continue with the derived `vX.Y.Z`.

3. **Major only: sweep the deprecation markers.** When the bump is a major, run
   the release-sweep skill *before* the tag — the deprecation disposition walk at
   the release boundary, its contract lifecycle-kit/SPEC.md §templates/skills/.
   Every marker on the roster earns a stamped disposition in
   `.workflow/release-sweep-evidence.txt` before the tag; an empty roster is a
   stated "none". No marker rides into the next major undispositioned. That file
   is a tracked checked projection of the workflow directory
   (gate-sdk/SPEC.md §The workflow directory), so its first line is the
   `# contract: RELEASING.md §The procedure — …` header carrying the block
   grammar; a disposition block opens with a bare `<release> — <date>` line
   below it, never a `#` one.

4. **Tag the iteration's final commit.** Tag `vX.Y.Z` on the iteration's final
   commit and push it — `git tag -a vX.Y.Z` on the commit, then push the tag to
   the origin. The closing session runs steps 4-6 itself when it holds the
   credentials (the default — an authenticated `gh` login carrying `repo` scope
   and a working `git push`, confirmed with `gh auth status`); only a genuinely
   keyless sandbox defers these to the operator, whose push mechanics live in
   the local ops runbook, outside the tree. Stamp `<iteration> release vX.Y.Z —
   <basis>` into the disposition evidence.

5. **Create the GitHub Release.** Its body points at the post's
   `https://checkwright.dev/` URL — the post is the note's single home, the
   Release a pointer to it, never a second copy of the note. Write the post URL
   **without a trailing slash** (`…/posts/<slug>`, not `…/posts/<slug>/`): the
   site serves the bare form and 404s the slashed one. Open the link once the
   Release is published — the body lives on the host, out of the battery's
   reach, so this verification is the only thing standing between a typo and a
   dead link in a permanent artifact.

6. **Verify the version badge.** Confirm the README release-version badge
   resolves the new tag. It is sourced from the GitHub tag list, so each release
   updates it with no edit — this step is a verification, not a write.
