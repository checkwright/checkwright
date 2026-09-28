# SPEC amendment: roadmap-motion

A roadmap horizon moves only when a consultation happens to re-tag it. The consult binding reads ROADMAP.md only as a projection and names no tag reconciliation. The roadmap arm prints an empty horizon as information (queue-kit/SPEC.md §The roadmap arm), scope reads the tag only as a ranking input, and no drain retires a tagged entry. `check-roadmap-fresh` compares the projection with the tags, never the tags with direction, so it stays green throughout. This amendment has close raise a consult-owed item when the roadmap's motion needs direction, and has this repo's consult binding reconcile the tags whenever direction changes.

One queue entry pairs it: [roadmap-horizon-motion-unowned](../TASK-QUEUE.md#roadmap-horizon-motion-unowned), blocked by [consult-inbox](../TASK-QUEUE.md#consult-inbox), whose `--emit file-consult` arm delta 1 calls. The blocked-by tag orders the two, so no delta has a before-landing act.

**The rulings.**

- **Close raises the signal and never re-tags.** A `[roadmap:]` tag is a public statement of direction, and direction is the operator's. Close holds the iteration's exits and its commit range, so it sees what left the roadmap. The consultation sets what replaces it.
- **Two conditions, both read off the roadmap arm.** An entry that left the roadmap during the iteration, by a Done move or a dropped tag; and a first configured horizon with no bullet now. The first compares the arm's output over the queue at the range's start with its output now, since the arm takes a queue-file operand. The second reads the output now. The kit names no horizon: *first configured* is the knob array's first member (`QUEUE_KIT_HORIZONS`).
- **One item per condition, never a repeat.** Close reads the consult inbox first and files nothing a standing item already names, so a vacancy that outlasts several closes is one owed question and not one per close.
- **The step skips where no roadmap is configured.** The arm exits 2 while `QUEUE_KIT_HORIZONS` is empty, and close reads that as nothing to read.
- **Tag reconciliation is this repo's binding content.** The horizon and track vocabulary is consumer config, and so is the choice to publish a roadmap. The kit's consult template stays silent, and this repo's consult binding gains the landing rule.
- **The seam.** Kit mechanism: the close step and its grounds. Consumer config: the roadmap vocabulary and the consult binding's reconciliation rule. No private rule content.

**Refused.**

- **Close re-tagging.** It would make the closing stage decide direction.
- **A gate reddening a vacant horizon.** queue-kit/SPEC.md §The roadmap arm already refuses gating the per-horizon band as a curation opinion, and a vacancy is a true state of the queue until the operator answers.
- **A freshness assertion of tags against direction.** Direction lives in no file a gate can read.

## What changes

### (1) Close reads the roadmap's motion {design-bearing}

**Not yet applied.** lifecycle-kit/templates/stages/close.md step 5: after *A `[roadmap:]` entry is outside every exit (queue-kit/SPEC.md §The icebox tier) and stays.* insert:

> **Then read the roadmap's motion.** Run `--emit roadmap` over the queue file as it stood at the range's start (`git show <start>:<queue-file>` into the scratch dir, passed as the arm's operand) and over the queue file now. Where the arm exits 2 for want of a configured horizon, skip the read. File one consult-owed item with `--emit file-consult` naming every slug the first output lists and the second does not, and one when the first configured horizon holds no bullet now, unless an item already in the consult inbox names it. Re-tag nothing: a consultation sets the direction.

lifecycle-kit/SPEC.md §templates/stages/, the close template subsection, gains a paragraph after the one opening **The sweep's candidates are derived**:

> **The roadmap-motion read** sits beside the moot sweep because both read the iteration's exits against its range. A `[roadmap:]` tag states direction publicly, and nothing else notices when the iteration removes one: a Done move drops every tag, the roadmap arm prints an empty horizon as information, and `check-roadmap-fresh` compares the projection with the tags, never the tags with direction. So close compares the arm's output at the range's start with its output now, and reads the first configured horizon for vacancy. It files each finding to the consult inbox (§The consult inbox) rather than re-tagging, since direction is the operator's. It reads the inbox before filing, so a standing vacancy is one owed question. **Honest limit:** a re-tag that moves an entry between horizons is not read, and nothing gates that the read ran.

### (2) The consult binding reconciles the tags {mechanical}

**Not yet applied.** `.claude/commands/consult.md`:

- **entry-reading**: *Read `ROADMAP.md` only as a projection of the queue, never as a second source.* stands, followed by: *A consult-inbox item naming a roadmap slug or a vacant horizon is answered by the reconciliation below.*
- **landing-surfaces** gains a bullet after the **Work** bullet:

  > - A direction change that moves what is next → the `[roadmap:]` and `[roadmap-summary:]` tags on every entry it concerns, re-tagged in this session and `ROADMAP.md` regenerated (`bash gate-sdk/bin/run-gates.sh --emit roadmap --write`), so the public page states the direction the session set. A vacant first horizon the session leaves vacant is recorded as such in the landing commit's message.

  and *A decision fitting none of the four* becomes *A decision fitting none of the five*.

## Producers and consumers

Probe: `git grep -n` over the tracked tree for `roadmap`, `QUEUE_KIT_HORIZONS` and `ROADMAP.md` in `lifecycle-kit/`, `.claude/commands/` and `queue-kit/SPEC.md`; and `--emit roadmap <file>` run over a revision of the queue, which printed its bullets.

- **The consult-owed roadmap item** (delta 1). Producer: the close session, at step 5, reachable at every close where `QUEUE_KIT_HORIZONS` is set, as it is in this repo. Consumer: the consult skill's inbox drain, and in this repo the binding's reconciliation (delta 2). Its prose names the slugs that left, or the vacant horizon; the consultation reads it as the item's disposition body.
- **The reconciliation** (delta 2). Producer: a consultation whose ruling or direction changes what is next. Consumers: `check-roadmap-fresh`, which reds a tag change without the regenerated page, so the `--write` lands in the same commit; the public roadmap page.
- **Roster-holding readers:** none; no name is minted.

## Existing sections updated

Roster probe: the `git grep` above.

- `lifecycle-kit/templates/stages/close.md` — step 5 (delta 1).
- `lifecycle-kit/SPEC.md` — §templates/stages/, the close template subsection (delta 1).
- `.claude/commands/consult.md` — entry-reading and landing-surfaces (delta 2).
- `docs/lifecycle-kit/SPEC.md` — the generated on-site mirror, regenerated with `bash gate-sdk/bin/run-gates.sh --emit docs-mirror --write` (delta 1).
- `.workflow/release-declarations.md` — Behavior changes gains a bullet led by **lifecycle-kit/templates/stages/close.md**: close files a consult-owed item when a `[roadmap:]` entry leaves the roadmap or the first configured horizon is vacant (delta 1).

## Retired spellings

- None — every delta adds or re-phrases prose; no name, path or token is retired.

## Definition of Done

- [ ] **Causal completeness** — every point of canon-kit/SPEC.md §The causal-completeness check holds for each new state, event, interface and obligation.
- [ ] **Instruction surfaces: instruction only** — replacement text for a template, agent definition or shim carries no grounds; a delta places them (the Content-tiering / SSOT rule).
- [ ] **Merged with no information lost** — each addition re-phrases the canonical-spec text it refines rather than appending to it; the merged spec reads as one document a reader who never saw the amendment can use alone.
- [ ] **Amendment deleted** — this file removed on merge; none remain for the component (`ls lifecycle-kit/SPEC-*.md`).
- [ ] **Removals propagated** — every name this change retired is declared in `## Retired spellings` above, and `check-amendment-retired-spelling` runs each declaration against the whole tracked tree, not against the specs alone.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed as debt tasks (a build-time causal gap is resolved that session, not deferred).
- [ ] **Battery green** — the full battery and lifecycle-kit's fixture suite green on the landing commit. The entry moves to Done before the drain stage (`LIFECYCLE_KIT_DRAIN_STAGE`).
