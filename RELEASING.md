# Releasing Checkwright

The repeatable release procedure — the **close stage's release-disposition
step** invoked at *every* iteration close (lifecycle-kit/templates/stages/close.md;
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
lifecycle-kit/SPEC.md §bin/enter-stage.sh). The line's grammar — every legal form
of the version field, including the deferral form step 2 below writes — is owned
by lifecycle-kit/SPEC.md §templates/stages/; cite it, never restate it here.

1. **Author the release-note post — in-iteration.** Add a dated `docs/posts/`
   entry carrying a `release: vX.Y.Z` front-matter key and every fixed section
   the upgrade contract's roster names; the skeleton below walks them in order.
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
   - **In brief (fixed, no `None` form)** — placed immediately after the
     Opener, ahead of the three variable sections below. Grammar owned by
     [docs/install.md](docs/install.md) §The upgrade contract; cite it, never
     restate it.
   - **The three variable sections** — Tightened gates, Renamed knobs, and
     Behavior changes, authored to [docs/install.md](docs/install.md) §The
     upgrade contract's grammar (a knob *removal* is expressed `old → ∅` under
     Renamed knobs); that pointer owns their grammar, this skeleton does not
     restate it. **Tightened gates is composed, not recalled** — its bullets come
     from `.workflow/tightened-gates.txt`, the declaration surface each build
     stage appended to as it landed or tightened a gate
     (gate-sdk/SPEC.md §upgrade-smoke). One bullet per declared name, each given
     the intent behind the move; an empty surface means a stated "None." Because
     the surface accumulates across every iteration since the last tag, a release
     batching several iterations inherits all of their declarations here.
     **Held by a gate, not by review:** `check-tightened-gates-note-parity`
     asserts the composed section's token set equals the surface it was
     composed from while the note is under composition (its declared version
     carries no tag yet), so the transcription this step performs is checked
     at commit time rather than trusted to a manual read-across.
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
   is gated by `check-release-bump`), read off the note's three variable
   sections — `## In brief` feeds no bump criterion, so do not look for one
   there. An
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
   the release boundary, its contract lifecycle-kit/SPEC.md §templates/release-sweep.md.
   Every marker on the roster earns a stamped disposition in
   `.workflow/release-sweep-evidence.txt` before the tag; an empty roster is a
   stated "none". No marker rides into the next major undispositioned. That file
   is a tracked checked projection of the workflow directory
   (gate-sdk/SPEC.md §The workflow directory), so its first line is the
   `# contract: RELEASING.md §The procedure — …` header carrying the block
   grammar; a disposition block opens with a bare `<release> — <date>` line
   below it, never a `#` one.

4. **Tag the iteration's final commit, and drain the tightened-gates declaration
   surface.** The
   drain and the disposition stamp are both tree writes, so the iteration's final
   commit is the one this step creates — which fixes the ordering, and **the
   ordering is not optional**. It binds in two places. First, **the note-authoring
   commit of step 1 precedes this drain-and-stamp commit, and the two are not to
   be squashed into one**: in the window between them the note and the surface are
   both non-empty and comparable, which is the only window in which
   `check-tightened-gates-note-parity` can hold them equal. Compose and drain in a
   single commit and the gate never sees a comparable state — it does not red, it
   simply has nothing to say, so the parity claim is silently forfeited.
   Enforcement of the split is this runbook's, not that gate's: a pre-commit gate
   cannot tell "note and drain in one commit" from "note authored while the
   surface was already empty". Second: write the **stamp** commit, push master,
   watch the `gates` run *for that SHA* go green, and only then tag that commit
   and push the tag. `CLAUDE.md` makes the remote oracle the authority over a
   master push, and a tag's whole purpose is to name an immutable tree other
   people fetch; tagging before the watch puts the tag on a tree only the local
   battery ever saw, inverting that authority invisibly — every gate is green
   either way, and the only tell is which SHA the `gates` run carries.

   **The drain lands after the tag, and the parity gate is what forces it.** That
   gate arms on a note whose declared version carries **no tag yet**, so while the
   note is untagged it holds the note's Tightened-gates set equal to the surface —
   which means draining before the tag reds it, with the whole composed set
   reported as gates that never tightened. Push and watch the stamp commit, tag
   it, push the tag; the note is then tagged, the gate goes dormant, and the drain
   commits cleanly. Sequenced this way the release is one `gates` watch and one
   `publish` watch, and the drain commit rides the next push rather than buying a
   third run.

   Tag with `git tag -a vX.Y.Z` on the stamp commit, then push the tag to the
   origin. The tag is also what discharges `.workflow/tightened-gates.txt`:
   step 1 composed the note from it, so drain it at the tag and only there — an
   iteration closing on `release none` or a deferral carries its declarations
   forward, which is exactly what the next release's note must inherit. Drain by
   **truncating to the header line**, never by clearing the file: it is a tracked
   checked projection whose header is required, and a whole-file clear reds
   `check-workflow-tiering` on the drain commit itself. Stamp
   `<iteration> release vX.Y.Z — <basis>` into the disposition evidence, in the
   commit the tag names.

   **The credential precondition — test the permission, not the scope.** The
   closing session runs steps 4-7 itself when it holds the credentials (the
   default); only a genuinely keyless sandbox defers these to the operator, whose
   push mechanics live in the local ops runbook, outside the tree. The property
   that decides it is the repository's own **`permissions.push` for the active
   account**, read with `gh api repos/<owner>/<repo> --jq .permissions`. The two
   things a session reaches for instead each prove nothing: `gh auth status`
   reports a token's **scopes**, and a scope is a *ceiling* on what a token may
   attempt rather than a grant of what the account may do on this repository; and
   a working `git push` over SSH uses a key and never consults the `gh` token at
   all, so it says nothing about API writes. Two clauses ride with it, placed
   here because a session will need them mid-release with a tag already public:
   - **A 404 on a write is a permission signature, not a missing object.** GitHub
     masks an unauthorized write as an absent resource, so a `gh release edit`
     returning 404 against a Release that plainly exists means *not permitted*.
   - **Resolve it by fixing the permission, never by switching identity.**
     Switching to another authenticated account gets past the 404 and leaves the
     real defect — an account that cannot write where this runbook assumes it can
     — in place and unrecorded.

5. **Watch the publish workflow — both channels.** Pushing the tag is what
   publishes the installer package: `.github/workflows/publish.yml` fires on the
   tag alone. It first **builds** one gate binary and one
   digest sidecar per target in the roster the build matrix is derived from
   (gate-sdk/SPEC.md §Consumer payload — no platform is spelled in the workflow).
   It then **assembles** the package once with `scripts/pack-installer.sh`, which
   verifies each artifact against its sidecar before placing it. Two sibling jobs
   then consume that one artifact — `release` attaches the tarball, the per-target
   binaries and every `.sha256` to the GitHub Release (the primary channel), and
   `npm` runs `npm publish --provenance` from the runner (the secondary one,
   held behind its approval environment). That hold is a **confirmation step
   that produces an approval record**, and claiming more would be an overclaim:
   the project is single-maintainer by standing ruling, so the reviewer and the
   tag pusher are one account and independent review is unavailable by
   construction. Expect the run to pause and to need your approval before the
   `npm` job starts. Nothing is published by hand, and
   **there is no version to edit** — the pack script stamps the version from the
   tag being packed and the job refuses a tarball whose stamp disagrees with it
   (docs/install.md §Versioning owns the one-semver-line rule this derives from).
   Watch **both** jobs to green (`gh run watch`) before continuing; a red publish
   is fixed and the tag re-pushed, never worked around by publishing locally. The
   job's credential and approval are repository configuration rather than tree
   state, so a first run on an unconfigured repository fails loudly on the
   missing token rather than publishing unattested. What the job may pass npm as
   a package spec is §The publish spec below, held by `check-npm-publish-spec`.

6. **Fill in the GitHub Release body.** Step 5's `release` job already created
   the Release and attached its assets, so this step writes the body rather than
   the Release. That body points at the post's
   `https://checkwright.dev/` URL — the post is the note's single home, the
   Release a pointer to it, never a second copy of the note. Write the post URL
   **without a trailing slash** (`…/posts/<slug>`, not `…/posts/<slug>/`): the
   site serves the bare form and 404s the slashed one. Open the link once the
   Release is published. The body lives on the host, out of the battery's reach,
   so its backstop is a monitor rather than a gate: `site-health.yml`'s
   release-body arm asserts daily that each note's Release body carries that URL,
   and separately that every apex URL the body *actually carries* resolves —
   two assertions over two different strings, which is exactly what catches the
   slashed form a presence check alone lets through — filing a `site-health`
   issue when either fails (site-kit/SPEC.md §templates/site-health.yml). That arm's latency is exactly
   why this hand-check stays — it is next-day and issue-shaped, while you are the
   only actor who can fix the body before anyone reads it. Verify by hand here;
   the arm is what catches the release where you did not.

7. **Verify the version badge.** Confirm the README release-version badge
   resolves the new tag. It is sourced from the GitHub tag list, so each release
   updates it with no edit — this step is a verification, not a write.

## The publish spec

The positional argument of an `npm publish` in a workflow must be unambiguously
a path **by its own literal text**. `check-npm-publish-spec` holds this over
`.github/workflows/`; the rule is npm's, and it is stated here because a spec
that reads correctly to a human is the shape that already reached a released
tag.

npm resolves a positional package spec as a **path** when it begins with `.` or
`/`, and as the GitHub shorthand **`owner/repo`** otherwise. The trigger is the
leading character, **not the slash** — `dist/x.tgz` sends npm to
`git ls-remote`, while `./dist/x.tgz`, an absolute path, and
`.tmp/pubrepro/dist/x.tgz` (three slashes, leading dot) are all read as paths.
So neither "contains no slash" nor "starts with `./`" is the rule, and a gate
narrowed to either would red a spec that works — including the `$PWD`-prefixed
form the workflow now carries.

A spec is unambiguous when, with **one layer of surrounding shell quoting
removed** (every real spec on this surface is quoted, so the strip comes first
or the gate reads `"` as the leading character):

- its first character is `.` or `/`; or
- it begins with a bare (`$PWD/`) or braced (`${PWD}/`) expansion of a
  proven-absolute root, immediately followed by `/`. The roster is exactly
  `PWD`, `GITHUB_WORKSPACE`, and `RUNNER_TEMP`, each absolute by a written
  contract — POSIX gives `PWD` as *an absolute pathname of the current working
  directory*, and the Actions default-environment-variable table documents the
  other two as runner-absolute. `HOME` is deliberately **not** on it: POSIX
  gives it as *a pathname* of the user's home directory with no absoluteness
  guarantee, and the Actions contract does not carry it at all. Re-proposing a
  root takes a contract citation, never a runner observation.

Everything else is refused, and two refusals look safe enough to name. A **bare
filename** publishes today only because a file of that name happens to sit in
the runner's cwd — a runtime property the literal does not express, which is why
the gate's message states the ambiguity and never predicts a failure. A
**command substitution** cannot be evaluated by any reader of the text, and is
the exact shape that shipped in `v0.16.0`; assign it to a variable and publish
through a prefixed expansion, which is what the `npm` job does.

The rule reaches `npm publish` alone. Its positional argument is by definition a
local package, so every one of them is path-intended; every other npm verb takes
registry specs as its ordinary case (`npm install lodash` is a correct bare
token), and applying this rule to them would red correct lines. Two reach limits
are deliberate: the check is line-local, so an `npm publish` continued across a
backslash is **refused loudly** rather than judged from a fragment; and shell
scripts under the tree are out of reach, because a `.sh` file's spec is
typically a variable whose absoluteness the text cannot prove — the workflow
surface is where the spec is written as a literal, which is what makes it
gateable there and nowhere else.
