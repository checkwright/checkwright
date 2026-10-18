# SPEC amendment: post-citation

Pairs `post-spec-cite-no-valve`. A dated post citing a kit SPEC section by heading reds `check-spec-pointer` once a later unit deletes that section: the posts are in `CANON_KIT_MANIFEST_FILES` (`docs/posts/*.md` in this repo), and the prose-citation pass carries no path valve. Every other canon gate's reading holds a post immutable, so an amendment rostering such a post as left standing could not be followed. The native-executable-git-hooks landing repointed the v0.30.0 post's `gate-sdk/SPEC.md §gen-pre-commit` to `§git-hook` with no rule licensing the edit.

**The rule: a stated duty to repoint, not a valve.** docs/page-authoring.md already reads a post as immutable *as prose* and "still link/command-resolved", and `CANON_KIT_TEMPORAL_EXEMPT_PATHS` is stated as taking "link and command resolution but not narration governance". A `§` citation is the same class as a link: a reference whose target another unit can retire. A valve would leave a published post sending its reader to a section that no longer exists, which link resolution already refuses for links. So the citation joins the resolved class, and the unit retiring its target owes the repoint. The cost is one line per retired heading a post cites, met once in the 39 lines carrying a `§` across the posts so far.

**Held at the entry's width, not the principle's.** The icebox's `post-immutability-machine-read-carveout` asks whether the carve-out names its element classes or states the principle, *immutable as prose, live as machine input*. Widening docs/page-authoring.md's governance sentence to name machine-read elements generally was offered and declined at the release-note lead-token grammar's spec. So this amendment adds the one class its entry concerns, the section citation, and leaves that question with its entry. Spec raised the width to the lead, and the operator chose this width (operator direction 2026-10-18, lead-relayed, not a ruling): the icebox entry stays dormant and is not paired here.

## What changes

### (1) A post's section citation is resolved, and the retiring unit repoints it {design-bearing}

`canon-kit/SPEC.md` §check-spec-pointer gains a paragraph after the one beginning "An adjacent path that is not a tracked file", the paragraph that closes the three citation forms' list. **Not yet applied.**

> **A dated post's citation is held like its links.** A file matching `CANON_KIT_TEMPORAL_EXEMPT_PATHS` is immutable as prose, yet stays in this pass. A unit retiring a heading such a file cites repoints the citation in the same commit: to the section that carries its content now, or to the file alone when none does. That repoint is the one edit such a file takes. There is no valve, because a published citation to a retired section dangles for every reader, as a dead link would.

Replacement for `canon-kit/SPEC.md` §check-manifest-temporal, the `CANON_KIT_TEMPORAL_EXEMPT_PATHS` valve bullet's last clause. **Not yet applied.**

> … dated announcement posts, which take link, command and section-citation resolution but not narration governance.

### (2) The merge step names the repoint {mechanical}

Replacement for `canon-kit/SPEC.md` §Merging an amendment, step 5. **Not yet applied.**

> 5. Propagate removals (grep every spec for names the change retired, and repoint a dated post's citation of a retired heading, §check-spec-pointer), file discovered gaps as debt tasks, and commit the merge with the work.

### (3) The site's page rule and this repo's knob comment say the same {mechanical}

`docs/page-authoring.md` §Page-authoring rules, the clause "dated `docs/posts/` are immutable, temporal-exempt but still link/command-resolved". **Not yet applied.**

> dated `docs/posts/` are immutable, temporal-exempt but still link-, command- and section-citation-resolved, a retired target's repoint being the one edit a post takes

`scripts/canon-config.knobs`, the `# comment-tier-exempt:` line above `CANON_KIT_TEMPORAL_EXEMPT_PATHS`: "while link and fenced-command resolution still apply" becomes "while link, fenced-command and section-citation resolution still apply". **Not yet applied.**

## Producers and consumers

- **The repoint duty (deltas 1 and 2).** Producer: a unit whose change deletes or renames a heading. Its enabling condition is a post citing that heading, which this repo's posts meet: `git grep -c "§" -- 'docs/posts/*.md'` over the tracked tree counts 39 lines carrying one. Consumer: `check-spec-pointer`'s prose-citation pass, unchanged, which reds the commit until the repoint lands. The merging session reads the duty at §Merging step 5. No new field, knob or name.
- **Roster-holding readers:** none. The duty mints no name. The probe was `git grep -n "link/command-resolved\|link and command resolution\|fenced-command resolution"` over the tracked tree outside the posts and the generated mirrors.
- **Point 5:** no delta narrows a corpus. The pass's corpus is unchanged; the delta states that it is deliberately unchanged.
- **Point 6:** no delta obliges the members of an enumerable corpus at authoring time. The duty binds a future retirement, and every post citation resolves today, the battery's `check-spec-pointer` being green at this amendment's base.

## Existing sections updated

The roster came from the probe above.

- `canon-kit/SPEC.md` §check-spec-pointer (delta 1).
- `canon-kit/SPEC.md` §check-manifest-temporal, the path-valve bullet (delta 1).
- `canon-kit/SPEC.md` §Merging an amendment, step 5 (delta 2).
- `docs/page-authoring.md` §Page-authoring rules (delta 3).
- `scripts/canon-config.knobs`, the temporal-exempt-paths comment (delta 3).
- `docs/canon-kit/SPEC.md`, the generated mirror (all deltas).

## Retired spellings

- None — the rule widens a clause and retires no name.

## Definition of Done

- [ ] **Causal completeness** — every point of SPEC §The causal-completeness check holds for each new state, event, interface and obligation.
- [ ] **Instruction surfaces: instruction only** — replacement text for a template, agent definition or shim carries no grounds; a delta places them (the Content-tiering / SSOT rule).
- [ ] **Merged with no information lost** — each addition re-phrases the canonical-spec text it refines rather than appending to it; the merged spec reads as one document a reader who never saw the amendment can use alone.
- [ ] **Amendment deleted** — this file removed on merge; none remain for the component (`ls canon-kit/SPEC-*.md`) once the iteration's last canon-kit amendment merges.
- [ ] **Removals propagated** — every name this change retired is declared in `## Retired spellings` above, and `check-amendment-retired-spelling` runs each declaration against the whole tracked tree, not against the specs alone.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed as debt tasks (a build-time causal gap is resolved that session, not deferred).
- [ ] **Battery** — the full battery runs green.
- [ ] **Done move** — the entry moves to Done in the commit that merges this amendment, before the drain stage.
