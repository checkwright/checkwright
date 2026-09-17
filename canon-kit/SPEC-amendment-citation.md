# SPEC amendment: amendment-citation

**An amendment is deleted when it merges, so a filename citation of one dangles from
that moment.** The damage lands where the citation is read as governing input: a
sibling amendment in the same iteration. The attested instance: an amendment named
two siblings by path, in present tense, as the specification of a mechanism it
consumed. Earlier build batches merged both siblings away, and the last batch had to
repoint two dead pointers by hand before it could reason from the file. Nothing fired.

**The owner-position question, and the decidable slice of it.** Prose may cite a
merged amendment as settled history. It may never cite one as a live owner. Tense
separates the two, and tense is not mechanically decidable. The spelling of the
citation is. Two corpora were measured on 2026-09-17:

- **Sibling amendments.** Every attested owner citation is the path form:
  `<component>/SPEC-<name>.md`, bare, in running prose. It sat in the amendments
  authored by one spec commit, recovered with `git show <commit>:<path> | grep`. No
  `§`-heading form occurs. So a `§`-form-only proxy would catch none of them. Inside
  an amendment, a filename citation of another amendment is *always* read as input,
  because the file is a transition artifact. Its durable history form is the
  canonical section the cited amendment merged into, which a reader can still open.
- **Queue bodies.** `TASK-QUEUE.md` bodies held seven filename citations of an
  amendment. Six are lawful history: each names a merged amendment as the place a
  past claim was made. One was an owner citation. `docs-link-red-remedy-first` said
  the merged prose-profile amendment "keeps" a gate `on-surface`. The pairing commit
  repoints it to the gate descriptor's `install:` line, which is the live owner. The
  slug-form owner citations the entry was filed on are gone. A path-form assertion
  over the queue would red on six lawful citations to catch one. That is the prose
  queue-kit/SPEC.md §The tag algebra protects ("valuable prose no gate may punish").

**So the assertion reads the amendment corpus and not the queue.** The queue half is
recorded as an honest limit. It is not a deferred assertion. Nothing decidable
separates its two populations.

**What stays as it is.** Arms (a) to (c) of `check-amendment-queue`, and its
best-effort finder posture. `check-spec-pointer`, whose corpus is the manifest set and
not the amendment set. The queue-edges arm and the tag algebra's unresolved-token
rule.

## What changes

### (1) check-amendment-queue gains arm (e): an amendment's amendment citations resolve

§check-amendment-queue and `native/src/gates/amendment_queue.rs` gain one arm. Every
amendment-glob filename cited in the body of an amendment on disk must resolve to a
file {design-bearing}. **Not yet applied.**

- **The token.** Take the basename pattern of `CANON_KIT_AMENDMENT_GLOB` (default
  `SPEC-*.md`), with its `*` matching one or more of `[A-Za-z0-9._-]`. Allow an
  optional repo-relative directory prefix of the same characters plus `/`. On both
  sides of the token must stand a character outside that class, or the line edge.
  Backticks around the token do not change the match. A placeholder such as
  `SPEC-<feature>.md` or the glob `SPEC-*.md` itself contains a character outside the
  class, so it is not a token.
- **Resolution** reuses arm (c)'s calibration for a `[spec:]` ref, so the two cannot
  diverge. A token containing `/` resolves when that repo-relative path is a file.
  A bare basename resolves when an amendment on disk carries that basename. An
  amendment citing its own name resolves.
- **Skipped:** fenced blocks, on the ground §The amendment lifecycle gives the fence
  (a quoted example is grammar being shown), and HTML comment blocks (template
  guidance text). Nothing else is skipped.
- **Red.** One line per unresolved token:
  `<amendment>:<line>: cites <token>, which names no file — an amendment is deleted on merge; cite the canonical section it merged into`.
  These print under the existing violation header. The help line gains the same
  remedy.
- **No valve.** Every filename citation of an amendment inside an amendment is read
  as governing input, and every history use has a durable form: the merged section.
  A per-site exemption would therefore only ever license the defect. An illustration
  that must show a filename goes in a fence.
- **Fail-closed** as arms (a) to (c): an unreadable amendment is exit 2. The finder
  stays best-effort. An empty amendment set makes (e) vacuous, which hides nothing,
  because with no amendments there is nothing to cite from.

### (2) The merge step and the coverage paragraph name the new obligation

§Merging an amendment step 3 gains a clause, and §check-amendment-queue's
authoring-consequence paragraph gains a second consequence {mechanical}.
**Not yet applied.** Re-phrase, never append:

- **Step 3** ("Delete the amendment file…") becomes: "Delete the amendment file, and
  in the same commit repoint every sibling amendment's filename citation of it to the
  canonical section it merged into (arm (e) reds the commit otherwise); verify none
  remain for the component." The none-remain sentence that follows stays as it is.
- **§check-amendment-queue**, after the arm-(c) authoring-consequence paragraph: the
  owner-position slice. Arm (e) reads the amendment corpus alone. The queue half is
  the stated honest limit, with the measured ground that its filename citations of
  amendments are history form. The slug-form owner citation is not decidable either.
  Both stay review's.

## Producers and consumers

- **Arm (e) (delta 1).** Producer: `check-amendment-queue`. Its input is the existing
  `spec_amendments` finder output, already enabled wherever the gate is installed
  (`install: zero-config`). Its descriptor's `couples=` already reach `SPEC-*.md`,
  `*/SPEC-*.md` and `knob:CANON_KIT_AMENDMENT_GLOB`, so no trigger widens. Consumers:
  the committing session through the output contract, on the pre-commit hook,
  `run-gates.sh` and CI; the merging build batch, which meets the red at the commit
  that deletes a cited sibling (delta 2 names the remedy); and the `--run-gate-tests`
  arm through the fixture pair. The finding's fields: `<amendment>:<line>` is read to
  open the site, `<token>` is read to find what the merged amendment became.
- **Roster-holding readers.** No knob, token or gate name is minted. The finding text
  is not a roster member anywhere. New `// spec:` comments bind to
  §check-amendment-queue and meet `check-comment-tier`'s existing directive shape.
- **Point 5 (narrowing).** Nothing narrows. Arm (e) adds a red condition over an
  existing corpus.
- **Point 6 (members).** The obliged corpus at authoring time is the amendments on
  disk, enumerated by `git ls-files --others --cached --exclude-standard '*SPEC-*.md'`
  minus `templates/` and `gate-tests/`. That gives this file and the two sibling
  amendments of this iteration, paired with `design-pending-tag-restates-its-own-section`
  and `queue-entry-evidence-tier`. Each one's satisfying value is "every filename
  citation of an amendment resolves". None of the three names another by filename,
  and none names a merged amendment by filename, so each is clean under arm (e)
  whatever the merge order.

## Existing sections updated

- `canon-kit/SPEC.md` §check-amendment-queue: the invariant gains arm (e) and its
  calibration (token, resolution, skips, no valve), plus the owner-position slice
  paragraph (deltas 1 and 2).
- `canon-kit/SPEC.md` §Merging an amendment: step 3 (delta 2).
- `native/src/gates/amendment_queue.rs`: arm (e), the help line, unit tests for the
  token boundaries (placeholder, glob, backticked, prefixed path) and the fence and
  comment skips (delta 1).
- `canon-kit/gate-tests/check-amendment-queue/bad/`: an amendment citing a gone bare
  basename and a gone path, and `expect.txt`'s two new lines (delta 1).
- `canon-kit/gate-tests/check-amendment-queue/good/`: the widget amendment gains a
  self-citation, a fenced gone citation and a `SPEC-<feature>.md` placeholder
  (delta 1).
- `canon-kit/checks/check-amendment-queue.gate`: the `# spec:` description names the
  amendment-citation arm (delta 1).
- `.workflow/release-declarations.md`: one bullet. The gate is tightened, so a
  consumer amendment citing a merged sibling by filename now reds (delta 1).
- `TASK-QUEUE.md`: `amendment-owner-position-citation` moves to Done at merge, by the
  build session that merges this file, before the drain stage is entered
  (all deltas).
- `docs/canon-kit/SPEC.md`: generated mirror, regenerated (all deltas).
- <!-- update-target-exempt: generated projections, each rostered with its freshness gate and regen command in docs/site-architecture.md §Generated projections and their freshness gates --> `docs/enforcement.md` and the generated pre-commit hook.

Roster produced by `grep -n "check-amendment-queue"` over `canon-kit/` and
`native/src/`, and `grep -n "Delete the amendment file"` over `canon-kit/SPEC.md`. It
is a floor that build re-derives.

**Merge coordination.** The sibling unit `design-pending-tag-restates-its-own-section`
rewrites this module's tag arms and this fixture pair. Whichever batch lands second
rebases onto the first. Arm (e) reads no tag and the retired-token arm reads no
amendment body.

## Retired spellings

- None — no delta retires a spelling; arm (e) is added beside the existing arms.

## Definition of Done

- [ ] **Causal completeness** — every point of the kit's causal-completeness check
      holds for arm (e).
- [ ] **Merged with no information lost** — the decidable-slice measurement, the
      no-valve ground and the queue-side honest limit survive in the merged prose.
- [ ] **Attested reproduction** — a scratch tree holding two amendments, one citing
      the other by path, is clean. Deleting the cited one reds arm (e) at the citing
      line.
- [ ] **Queue move placed before the drain stage** — the Done move lands in the
      session that merges this file, before the drain stage is entered.
- [ ] **Amendment deleted** — this file removed on merge; none remain for the
      component (`ls canon-kit/SPEC-*.md`) once the iteration's last batch lands.
- [ ] **Gaps filed** — cross-component gaps found during the work go to the gap inbox.
