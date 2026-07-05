# SPEC amendment: <feature-name>

<!--
  A SPEC amendment is the delta artifact for a designed-but-unimplemented
  change. Name it after the feature (SPEC-<feature>.md, e.g. SPEC-sqlite.md —
  never SPEC-PHASE3-SQLITE.md), place it in the owning component's directory
  (a governance/workflow ruling with no owning component lives at the repo
  root), and pair it with a queue entry tagged [spec: SPEC-<feature>.md]. It
  describes ONLY what is added or changed — do not restate the canonical spec.
  It is a transition artifact: merged into the canonical spec and deleted when
  the work completes (an amendment never outlives its implementation).
-->

## What changes

<!-- The delta: new invariants, states, interfaces, error behavior. Each will
     land in its proper canonical-spec section at merge — write it so it can. -->

## Producers and consumers

<!-- The causal-completeness check. For every new state, event, and interface:
     — Producer: the code path / call / timer that triggers it (and the enabling
       config some deployed configuration actually sets — not test-only).
     — Consumer: the component that receives it, by what mechanism.
     — Every field has a named reader: for each field on a new message, the
       consumer that reads it and the transition where it is read (a field with
       no reader is removed). -->

## Existing sections updated

<!-- Any canonical-spec section describing the prior flow that this change
     touches — updated here, in the amendment, not left to drift. -->

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
