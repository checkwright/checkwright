# SPEC amendment: <feature-name>

<!--
  A SPEC amendment is the delta artifact for a designed-but-unimplemented
  change. Name it after the feature (SPEC-<feature>.md, e.g. SPEC-sqlite.md —
  never SPEC-PHASE3-SQLITE.md), place it in the owning component's directory
  (a governance/workflow ruling with no owning component lives at the repo
  root), and pair it with a queue entry tagged [spec: SPEC-<feature>.md]. The
  basename need not match that entry's slug: the ref resolves as a bare
  basename tree-wide, and swapping a long slug into the tag can push the
  entry's lead line past the queue's wrap budget — so a shorter basename than
  the slug is the fix, not a mismatch to avoid. It
  describes ONLY what is added or changed — do not restate the canonical spec.
  It is a transition artifact: merged into the canonical spec and deleted when
  the work completes (an amendment never outlives its implementation).
-->

## What changes

<!-- The delta: new invariants, states, interfaces, error behavior. Each will
     land in its proper canonical-spec section at merge — write it so it can.

     Each delta is a heading of the form `### (<N>) <title>` — <N> a positive
     integer, no leading zero, numbering from 1, sequential and unique within
     this file. A gap or a repeat means a delta was split or dropped without its
     citations moving. Renumber a delta and you move its citations with it.
     Checked by check-amendment-update-target. -->

## Producers and consumers

<!-- The causal-completeness check. For every new state, event, and interface:
     — Producer: the code path / call / timer that triggers it (and the enabling
       config some deployed configuration actually sets — not test-only).
     — Consumer: the component that receives it, by what mechanism.
     — Every field has a named reader: for each field on a new message, the
       consumer that reads it and the transition where it is read (a field with
       no reader is removed).
     — Narrowing a corpus? name each reader's RED CONDITION, not its subject:
       only a monotone verdict is clearable by inspection (SPEC §The
       causal-completeness check, point 5). -->

## Existing sections updated

<!-- Any canonical-spec section describing the prior flow that this change
     touches — updated here, in the amendment, not left to drift. Each entry
     names the delta that owns it; an update target no delta claims reaches
     build as an orphan a batch adopts on its own authority.

     A citation is `delta <N>` or `deltas <N>` (case-insensitive), continuing
     into further integers through commas and/or the word `and` — `(deltas 2, 3)`,
     `(deltas 1 and 4)`; `delta 3's` is the same citation. `all deltas` cites
     every delta this file defines, for a target (a generated mirror, say) that
     goes stale the moment any of them lands. For a target deliberately owned by
     no delta, tag `<!-- update-target-exempt: <reason> -->` on the bullet's first
     line or the one above — the reason is mandatory. Checked by
     check-amendment-update-target. -->

<!-- The one sanctioned copy exemption: an amendment may embed a wire-contract
     delta (e.g. a fenced proto block) until merge, because it is the design
     home for a contract that does not exist yet. The canonical spec cites the
     contract file, never re-embeds it. -->

## Definition of Done

- [ ] **Causal completeness** — every new state/event/interface has a named,
      reachable producer and a named consumer; every new field has a named
      reader at a named transition.
- [ ] **Merged with no information lost** — each addition integrated into its
      proper canonical-spec section (not appended); the merged spec reads as one
      coherent document a reader who never saw the amendment can use alone.
- [ ] **Amendment deleted** — this file removed on merge; none remain for the
      component (`ls <component>/SPEC-*.md`).
- [ ] **Removals propagated** — grepped every spec for names this change
      retired; nothing dangles.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed as
      debt tasks (a build-time causal gap is resolved that session, not
      deferred).
