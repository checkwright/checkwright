# SPEC amendment: probe-corpus

The Probe-before-assertion rule asks whether a probe was *run*. The failure it now has to reach is a probe that ran over a **corpus narrower than the claim it supported**: a path-scoped grep behind a tree-wide claim, a filter whose empty return was read as success, a printed probe that over-returned, a confirming run over an inconsistent tree. The rule's coverage clause already names two members of that class, the keyword sweep and the bounded window. It does not name the class, so the path-scoped probe, the commonest member, reads as covered by a rule that says nothing about it.

**The ruling: widen the existing rule, and mint no new one.** The coverage clause is rewritten around the corpus: a probe supports a claim no wider than the corpus it ranged over, so the claim's corpus is named first and the probe run over that corpus. The digest takes the same widening, because the digest is the only part of the rule a session reads at every start, and the misses happened with the unwidened digest resident. A fourth always-loaded line was weighed and turned down: it would state the same rule twice, one copy narrower, which is the restatement the Content-tiering rule forbids.

**Mechanizing it is refused, on a stated ground.** Whether a probe's corpus covered its claim needs a reading of what the claim ranges over, and that reading lives in prose. The two places a claim is written in a form a tool reads already carry both halves: an amendment's update-target roster names the probe command and the corpus it ran over (canon-kit/SPEC.md §check-amendment-update-target), and a survey record block names its corpus and oracle side by side (lifecycle-kit/SPEC.md §The survey record). A check that the oracle's pathspec covers the declared corpus would compare two strings one author wrote, so it would hold the record to itself and not to the claim. The rule stays judgment with its capture mechanism, on the Enforcement-first false-positive carve-out.

**Measured at authoring (2026-09-22).** `git grep -n -c "Probe-before-assertion" -- ':!TASK-QUEUE.md'` finds the rule's name in `CLAUDE.md` (the digest), `doctrine-kit/DOCTRINE.md` (the rule, and the third-party-tool rule's divide) and the generated mirror `docs/doctrine-kit/DOCTRINE.md`. `git grep -n -F "a claim one cheap command would settle"` finds the digest text in `CLAUDE.md`, `doctrine-kit/DOCTRINE.md` and the mirror, and nowhere else.

## What changes

### (1) DOCTRINE.md, Probe-before-assertion: the coverage clause is rewritten around the corpus {design-bearing}

**Not yet applied.** In `doctrine-kit/DOCTRINE.md`, rule Probe-before-assertion, replace the three sentences from "**A probe that ran can still fail to settle the claim, and its silence reads exactly like an answer.**" through "…or the section's real extent against the window." with:

> **A probe supports a claim no wider than the corpus it ranged over, and its silence reads exactly like an answer.** A path-scoped grep settles its path, not the tree. A filter's empty return means success only if the filter could have matched. A keyword sweep misses prose that describes a thing without naming it, and a bounded window (`grep -A`, a line range) says nothing past its edge. None of these reports its own coverage. So name the claim's corpus first, run the probe over that corpus, and check the probe's coverage before reading its silence as a negative: a second sweep on a paraphrase, the section's real extent against the window, the size of the set it ranged over. A probe over a narrower corpus is a stand-in for the claim's subject, the case the Read-the-subject rule governs.

### (2) DOCTRINE.md, Probe-before-assertion: the *Enforced by* sentence states the refusal {mechanical}

**Not yet applied.** The last sentence of the rule's *Enforced by*, "Whether any slice of the class is mechanizable is open.", becomes: "Mechanizing the corpus check is refused: the two places a claim's corpus is written for a tool, an amendment's update-target roster and a survey record block, write the corpus and the probe side by side, so a check comparing them would hold the record to itself rather than to the claim."

### (3) The digest names the corpus {mechanical}

**Not yet applied.** The rule's `*Digest:*` trailer becomes:

> a claim one cheap command would settle is probed over the corpus the claim ranges over before it is asserted; relaying an unverified premise is asserting it.

`CLAUDE.md`'s `## Delivery doctrine` bullet for Probe-before-assertion takes the new text verbatim in the same commit, since check-doctrine-registration assertion F reds a bullet that differs from its trailer. Regenerate `docs/doctrine-kit/DOCTRINE.md` with the command its freshness gate prints. The bullet grows `CLAUDE.md`, so the growing commit carries the ceiling re-stamp `check-surface-ratchet` prints.

The sibling `consumer-policy-rule-absent` inserts a methodology rule and renumbers the craft register. This amendment cites rules by name only, so it is unaffected whichever batch lands first.

## Producers and consumers

- **The widened clause and refusal** (deltas 1 and 2). Producer: the doctrine file, read by every session that follows the digest's link, and by the stages whose templates run the probe pass (lifecycle-kit/SPEC.md §templates/stages/). Consumer: the session writing a claim. No gate reads the body text.
- **The widened digest** (delta 3). Producer: the `*Digest:*` trailer. Consumers: `--install-doctrine`, which derives the bullet from it (doctrine-kit/SPEC.md §install-doctrine), and `check-doctrine-registration` assertion F, which reds the agent file's bullet until it matches verbatim.
- **Point 5.** Not reached; no corpus narrows.
- **Point 6.** Not reached.

## Existing sections updated

Roster from `git grep -n -F "a claim one cheap command would settle"` and `git grep -n -c "Probe-before-assertion" -- ':!TASK-QUEUE.md'`, run 2026-09-22.

- `doctrine-kit/DOCTRINE.md` rule Probe-before-assertion: the coverage clause (delta 1), the *Enforced by* sentence (delta 2) and the `*Digest:*` trailer (delta 3).
- `CLAUDE.md` §Delivery doctrine, the Probe-before-assertion bullet (delta 3).
- `.workflow/release-declarations.md`, a Behavior changes bullet: a re-vendored doctrine's changed digest reds `check-doctrine-registration` assertion F on an adopter's agent file until `--install-doctrine` re-runs (delta 3).
<!-- update-target-exempt: generated mirror, regenerated by its freshness gate's printed command -->
- `docs/doctrine-kit/DOCTRINE.md`.

## Retired spellings

- `a claim one cheap command would settle is probed before it is asserted` — the digest's unwidened text (delta 3).

## Definition of Done

- [ ] **Causal completeness.** Every point of canon-kit/SPEC.md §The causal-completeness check holds for the widened clause and digest.
- [ ] **Instruction surfaces: instruction only.** The `CLAUDE.md` bullet carries the digest and nothing else.
- [ ] **Merged with no information lost.** The coverage clause is re-phrased, not appended to; the keyword sweep and bounded window survive inside the new sentence set.
- [ ] **Amendment deleted.** This file is removed on merge (`ls doctrine-kit/SPEC-*.md`).
- [ ] **Entry moved.** `probe-before-assertion-doctrine` moves to Done in the merge commit, at a stage before the drain stage.
- [ ] **Removals propagated.** `check-amendment-retired-spelling` is green on the declared spelling.
- [ ] **Gaps filed.** Any cross-component gap found during the work is filed to the gap inbox.
