# SPEC amendment: the pack step's clean-tree predicate

The joined half of `packer-port-terminal-cut`: the packer's refusal stops being a
whole-tree dirty check and becomes a check on the paths the payload is actually
assembled from. It is joined to `pack-installer-cut-to-a-non-gate-arm` on
producer/consumer rather than adjacency — the cut rewrites the file this defect
lives in, so cutting first and deciding the predicate later ports a known defect
into the new arm.

**This amendment is authored against the arm the sibling cut mints, not against
the shell form.** The two land in one iteration and the cut lands first; every
delta below is stated substrate-neutrally so that it is the same change either
way, and the one place the ordering matters is named at §Producers and consumers.

**The candidate shape is chosen here, which is what `[design-pending]` was
recording.** The entry named three; delta 2 takes the first and records why the
other two are refused.

## What changes

### (1) §The packer states the payload's tree footprint, and it is derived rather than listed

The section gains a statement of **which paths of the packed tree the payload
depends on**, in three members, because a scoped refusal is unstateable without
one — a wrong scoping silently under-refuses on a genuinely dirty path, which is
the failure the refusal exists to prevent. {design-bearing}

- **The tracked set at the stamped commit**, under `installer/` and under each
  root the kit-root resolver yields. Its bytes come from `git archive` at that
  commit, so they are the **commit's** bytes and no worktree edit can change
  them. This is the member a reader is most likely to assume is at risk and it is
  the one member that never is.
- **The kit-root set itself**, which is decided in the worktree rather than in
  the commit: the resolver admits a root by testing for a `checks/` or `smoke/`
  directory on disk, and the pack loop tests the root's own existence again. A
  tracked kit root deleted in the worktree is therefore **dropped from the
  payload while the stamp names the commit that carries it** — a real divergence
  between the payload and its stamp, and the sharpest reason the refusal is not
  simply redundant.
- **The native target roster**, which the pack copies verbatim out of the
  worktree when `--artifacts` is given. Where the roster knob resolves to a path
  **inside** the packed tree, its dirty bytes ship under a stamp that does not
  describe them; where it resolves outside the tree — the Windows leg steers it
  to a scratch file — no clean-tree check ever covered it and none can.

What is **not** a member: the `--artifacts` tree, which is outside the worktree
by contract; and every other path in the repository, which the payload neither
ships nor reads.

The statement is **derived, never transcribed**: the pathspec is computed from
the same two resolvers the pack loop itself uses, so the refusal's corpus and the
packed set cannot drift apart. A hand-maintained list here would be the
under-refusal hazard reintroduced as a maintenance burden.

### (2) The clean-tree refusal is scoped to that footprint

The refusal asks `git status --porcelain` **restricted to the footprint's
pathspec** — `installer/`, each resolved kit root that lies inside the packed
tree, and the roster path when it lies inside it — rather than about the whole
worktree. A root that resolves outside the packed tree is dropped from the
pathspec rather than passed to git, which refuses a path outside the repository.
{design-bearing}

**Untracked-but-not-ignored paths inside the footprint still refuse.** They do
not ship — the pack reproduces the tracked set — and they cannot make the stamp
wrong, so the conservative reading needs its own ground and has one: an untracked
file under a packed root means the tree under test and the tree that will be
packed differ, which is precisely what the consumer smoke exists to notice.
Ignored paths stay invisible here as they always have, which is why the pack
reproduces the tracked set with `git archive` rather than copying the directory.

**Two properties are preserved and one is dropped, deliberately.** Preserved: the
stamp describes the payload, because the only two paths on which it could fail
are members two and three and both are inside the pathspec. Preserved: the
consumer smoke asserts on the tree it means — an edit to a shipping path still
refuses rather than being quietly replaced by the commit's bytes. Dropped: a
dirty path that the payload neither ships nor reads no longer aborts the pack,
and with it a whole validate battery. The attested instance is
`.workflow/gap-inbox.md` — the one artifact the repository's always-loaded
manifest instructs every mid-iteration session to write — which aborted a
validate run after sixteen of twenty-two suites had already passed clean.

**The three refused alternatives, with the grounds they are refused on.**

- **Ruling gap-inbox commit ownership so the file is never dirty across a stage
  boundary.** Refused: it answers one dirty path and leaves the predicate wrong
  for every other one, and the question it turns on is independently owned by
  `gap-inbox-commit-ownership`, which this amendment does not touch and does not
  need answered.
- **Stating the pre-flight valve as the sanctioned response.** Refused: it makes
  overriding a correct refusal the routine move, which is the habit that gets a
  genuinely dirty tree packed. A valve that fires on every ordinary iteration
  stops being read.
- **Deriving the kit-root set from the stamped commit instead of the worktree**,
  which would shrink the footprint to two members and make member two immune the
  way member one is. Refused on criterion 6: the kit-root derivation is shared
  library mechanism the whole battery runs on, and a second, commit-scoped
  derivation inside the pack path would be a duplicate with nothing holding the
  two together. The scoped refusal covers the same divergence at no such cost.

### (3) The refusal names the paths it found, not merely the tree

The diagnostic prints the offending entries — bounded, with a total — beside the
tree it checked, so a reader can tell a shipping path from scratch without
re-running `git status` by hand. The existing help lines are kept: the check is
still made once per invocation against the tree as it is now, and committing or
stashing is still the remedy. {mechanical}

### (4) The section's own stated reason is corrected where it over-reaches

§The packer today grounds the refusal in the commit stamp alone — *a dirty tree
would stamp a commit the payload does not describe*. That sentence is true of
members two and three and **false of member one**, which is most of the packed
set, and the gap between the stated reason and the actual predicate is what let
the predicate stay wider than anything it protects. The paragraph is rewritten to
state both grounds it actually has: the stamp, on the two members where the
worktree can reach the payload; and the tree-under-test property, for the
consumer smoke. {design-bearing}

## Producers and consumers

**No new state, event or interface is introduced.** This amendment narrows the
input corpus of one existing refusal and rewrites one governed paragraph, so
there is no new message, no new field and nothing to give a reader.

| changed behavior | producer | consumer | transition |
| --- | --- | --- | --- |
| the scoped clean-tree refusal (deltas 1, 2) | the pack arm, before the commit stamp is resolved | its seven call sites — two workflow steps and five consumer-smoke scenarios | once per pack invocation |
| the paths-named diagnostic (delta 3) | the same refusal path | the same callers' captured stderr, and a reader of a finished CI log | on refusal only |

**The producer's enabling configuration is already emitted everywhere it must
be.** The pathspec is computed from the kit-root resolver and the target-roster
knob, both of which every call site already resolves today — the Windows leg
steers the roster knob explicitly and the scoped pathspec must honour that
steering, which delta 1's third member is what states.

**The ordering this amendment depends on, stated once.** Its deltas are
implemented in the arm the sibling amendment mints. The cut lands first; if a
batch reaches this amendment before the cut has landed, the same three deltas
land in the shell form and move with it, which costs one rewrite and changes
nothing about the deltas themselves.

**Existing integration prose is updated in this amendment**, not left to drift —
the roster below is that update, and delta 4 is the replacement ground for §The
packer's clean-tree paragraph. **Not yet applied**: this stage authors and build
lands, so every passage named below is a proposal until the build stage merges it.

### Each reader's red condition, because this amendment narrows a refusal

No delta here narrows a **gate's** corpus, so canon-kit's point 5 has no
non-monotone gate reader to enumerate. It does narrow a **refusal's** corpus, and
the same reasoning applies to the callers that read its verdict, so they are
enumerated on the same terms rather than waved past.

- **The consumer smoke's pack scenarios** — red when the pack exits non-zero.
  Monotone in the dirty set: a narrower refusal can only turn a red run green.
  The property the smoke actually needs is not the refusal's width but that a
  shipping path's edit still refuses, which delta 2 preserves by construction.
- **The planted-roster scenario** — the one reader here whose verdict reds on
  **finding** a target rather than on finding none, which the smoke's own
  comment already records. It is untouched: this amendment changes no roster
  handling and plants its own file outside the tree.
- **The `pack:` job of the publish workflow** — red when the pack exits non-zero,
  and its checkout is clean by construction. Monotone, and unreachable either way.
- **The Windows pack leg** — same shape, with the roster knob steered outside the
  tree. Monotone, and delta 1's third member is what keeps the pathspec correct
  under that steering rather than passing git a path it will refuse.
- **`check-comment-tier`** — reds on a comment that is not a directive. Delta 3
  and delta 4 rewrite comment and prose text rather than adding restatement, and
  the `# spec:` line above the refusal moves with the paragraph it cites.

## Existing sections updated

- `installer/README.md` §The packer — the clean-tree paragraph is replaced: the
  footprint statement, the scoped predicate, the two preserved properties and the
  one dropped, and the three refused alternatives (deltas 1, 2 and 4).
- `native/src/emit/pack_installer.rs` — the arm the sibling amendment mints: the
  pathspec derivation, the scoped status call and the diagnostic (deltas 1, 2, 3).
- `scripts/pack-installer.sh` — the same three deltas, if and only if a batch
  reaches this amendment before the sibling cut has deleted the file (deltas 1,
  2 and 3).
- `.github/workflows/publish.yml` — the `pack:` step's comment grounding scratch
  and output placement in the whole-tree refusal; the practice is unchanged and
  its stated reason narrows with the predicate (delta 2).
- `TASK-QUEUE.md` — `binding-intel-leg-failed-one-run-in-two`'s distinctness
  sentence names this unit's subject as *the pack step's silence*, where the
  deliverable is the refusal's scope and the silence is the recurrence evidence;
  the sentence is corrected in the commit that lands this amendment's work
  (delta 4).

*No generated projection is in this amendment's roster, and that is checked
rather than assumed:* `installer/README.md` has no site mirror under `docs/`, and
none of the four surfaces above feeds one.

## Retired spellings

- None — this amendment narrows one predicate and rewrites the prose that grounds
  it; the flag roster, the scratch-base knob, the `PACK:` line and every
  diagnostic prefix are unchanged, so no name leaves the tree.

## Definition of Done

- [ ] **Causal completeness** — the scoped refusal's producer and its seven
      callers are named above; no new field is introduced, so none is unread;
      every caller's red condition is enumerated rather than assumed monotone.
- [ ] **Merged with no information lost** — the footprint statement and the three
      refused alternatives land in `installer/README.md` §The packer as prose in
      its proper place, not appended, and the section reads as one document.
- [ ] **Amendment deleted** — this file removed on merge; none remain for the
      component (`ls installer/SPEC-*.md`).
- [ ] **Removals propagated** — the negative retired-spellings declaration above
      is re-checked by `check-amendment-retired-spelling` at merge.
- [ ] **The predicate is witnessed, not asserted** — a dirty path outside the
      footprint packs clean and a dirty path inside it refuses, both exercised
      from the consumer smoke rather than by hand.
- [ ] **Gaps filed** — cross-component gaps discovered during the work resolved
      that session, not deferred.
