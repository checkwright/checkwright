# Releasing Checkwright

The repeatable release procedure. Checkwright carries one semver line applied as
git tags, the kits moving in lockstep — the versioning model is
[docs/install.md](docs/install.md) §Versioning, and the two-phase contract a
release serves is that page's §The upgrade contract. Every release follows the
ordered steps below; the release-note post is the note's single home and the
GitHub Release points at it.

Like [CONTRIBUTING.md](CONTRIBUTING.md), this runbook is governed repo-meta:
tracked, pinned in `scripts/core-files.list`, and in the spec manifest so its
links and commands resolve under the doc gates.

## The procedure

1. **Sweep the deprecation markers.** Run the release-sweep skill — the
   deprecation disposition walk at the release boundary, its contract
   lifecycle-kit/SPEC.md §templates/skills/. Every marker on the roster earns a
   stamped disposition in `.workflow/release-sweep-evidence.txt` before the tag;
   an empty roster is a stated "none". No marker rides into the next major
   undispositioned.

2. **Author the release-note post.** Add a dated `docs/posts/` entry carrying a
   `release: vX.Y.Z` front-matter key and the two sections the upgrade contract
   names — tightened gates and renamed knobs. The parseable grammar
   (front-matter key, section names, bullet lead tokens) is owned by
   [docs/install.md](docs/install.md) §The upgrade contract; "none" is a valid
   section body and is stated, never omitted. Add the post's path to
   `scripts/docs-offnav.list` — off-nav by design, its inbound links the GitHub
   Release and the render-time [releases index](docs/releases.md), which the
   note joins by its `release:` key with no further step. Launch-adjacent copy
   uses the reserved phrasing, "the verification layer under agent
   orchestration".

3. **Tag the release commit.** Choose the bump by
   [docs/install.md](docs/install.md) §Versioning's criteria (the derivable
   floor is gated by `check-release-bump`). Tag `vX.Y.Z` on the release commit and push it —
   `git tag -a vX.Y.Z` on the commit, then push the tag to the origin. Push
   mechanics for a keyless agent sandbox live in the operator's local ops
   runbook, outside the tree.

4. **Create the GitHub Release.** Its body points at the post's
   `https://checkwright.dev/` URL — the post is the note's single home, the
   Release a pointer to it, never a second copy of the note.

5. **Verify the version badge.** Confirm the README release-version badge
   resolves the new tag. It is sourced from the GitHub tag list, so each release
   updates it with no edit — this step is a verification, not a write.
