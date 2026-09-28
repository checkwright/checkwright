# SPEC amendment: consult-drain

The consult inbox (lifecycle-kit/SPEC.md §The consult inbox) is drained only when an operator remembers to run a consultation. This amendment adds the status cue and the drain trigger: a status-line counter for the inbox, and a lead that dispatches a consultation for inbox items while no stage session is live. It also states, rather than assumes, the classification question a dispatched consultation raises.

One queue entry pairs it: [consult-inbox-drain-trigger](../TASK-QUEUE.md#consult-inbox-drain-trigger), blocked by [consult-inbox](../TASK-QUEUE.md#consult-inbox). Every delta here reads the consult inbox's landed text and arm, so the blocked-by tag orders the two; no delta has a before-landing act.

**The rulings.**

- **The counter is a consumer-listed set of inboxes, off by default.** delegation-kit may not name a lifecycle-kit path, so the paths come from a knob. The status-line arm already reads the lifecycle state file by a hardcoded path, its stated existing style for a component every consumer's bar shows. An inbox counter is optional, so it takes a knob that defaults empty and renders nothing unset. The knob carries labels as well as paths, so the gap inbox can join it with no second knob.
- **Two consultations, split by trigger.** An operator-started consultation stays interactive and unchanged. A lead dispatches one for inbox items only, and only while no stage session is live, because a consultation writes the ruling record and the queue and the lead's write bar keeps the tree quiet under a live stage.
- **A dispatched consultation decides only what needs no operator.** It re-classes or discards an item alone, with the cause in its commit. A ruling-class item is escalated to the lead as a four-header block and stays in the inbox until an answer lands.
- **The batching threshold and the agent type are a lead-template slot.** No mechanism reads either value, so a knob would have `check-knob-citation` as its only reader, the ground lifecycle-kit/SPEC.md §Layout and configuration gives for carrying no ruling-authority knob. The slot's default is off: with no batching, the lead dispatches at the first quiet point after any item lands.
- **A dispatched consultation is not a stage-session type.** The workflow-state guard refuses a write by a caller of a type in `LIFECYCLE_KIT_STAGE_SESSION_TYPES` until that caller's id is stamped, and a consultation stamps nothing (lifecycle-kit/SPEC.md §check-dispatch-entry). So the slot names its own type.
- **The open question is stated with an interim.** Whether an operator answer relayed through a lead into a dispatched consultation is a ruling is left open on the entry's operator direction. The interim class is direction, because the relay rule refuses inflation. A ruling-class item therefore stays owed to a consultation the operator runs.
- **The seam.** Kit mechanism: the counter knob and its rendering, the lead's dispatch rule and slot, the consult template's dispatched mode, the open question. Consumer config: this repo's counter list, its slot binding and its consult agent definition.

**Refused.**

- **A threshold knob.** It has no mechanical reader; the lead reads the slot.
- **Dispatching a consultation beside a live stage.** It writes the ruling record and the queue, and both are surfaces a live stage's clean-tree preconditions refuse.
- **The dispatched consultation landing a ruling on a relayed answer.** That answers the open question in one direction, which the operator direction bars.
- **Rendering a zero count.** A `C0` on every bar is noise; an absent label and an empty inbox read the same, and the brief's count line says which.

## What changes

### (1) The status-line inbox counters {design-bearing}

**Not yet applied.** delegation-kit/SPEC.md §The statusline arm gains, after the counter-group paragraph:

> **The inbox counters** are a second group, appended after the queue counter group, one `<label><n>` token per `DELEGATION_KIT_STATUSLINE_INBOXES` element whose file holds at least one bullet. The count is the file's lines opening `- `, the one grammar both lifecycle-kit inboxes share. A zero count, an absent file and an unreadable one render nothing, and an empty knob renders no group. The kit names no path: which inboxes a consumer counts, and under which label, is that consumer's.

Its first sentence is amended: *and a queue counter group* becomes *a queue counter group, and any configured inbox counters*. `native/src/hook/statusline.rs` renders the group after the queue group, joined by `·`, and its `KNOBS` roster gains the knob.

delegation-kit/SPEC.md §Layout and configuration gains:

> - `DELEGATION_KIT_STATUSLINE_INBOXES` — array of `<label>=<path>` elements, each a repo-root-relative bullet file the status line counts (§The statusline arm); default empty, which renders no inbox group. The table validator refuses an element with no `=`, an empty label or an empty path, at exit 2.

`native/src/knobs/delegation_kit.rs` gains the row, `Row::indexed` with an empty default, and the validator check.

### (2) The lead dispatches a consultation {design-bearing}

**Not yet applied.** lifecycle-kit/templates/lead.md §The escalation protocol gains a paragraph after the one opening **One class the lead never rules, under either posture.**:

> **Dispatch a consultation for the consult inbox only while no stage session is live.** At a point where no dispatched stage session is live and the inbox (`LIFECYCLE_KIT_CONSULT_INBOX_FILE`) meets the slot's threshold, dispatch the consult skill as the agent type the slot names, its prompt naming the inbox items and nothing else. It escalates to you as a stage session does. Relay each ruling-class escalation to the operator and relay the answer back, stating its class. Dispatch no stage session while it is live.
>
> *<consult-dispatch: the agent type a lead dispatches a consultation as — never a stage-session type — and the batching threshold on the consult inbox: an item count, an oldest-item age, or off, which dispatches at the first quiet point after any item lands. Or the statement that this consumer's lead dispatches no consultation.>*

lifecycle-kit/SPEC.md §templates/lead.md gains one paragraph:

> **The lead drains the consult inbox by dispatch, in a quiet tree.** A consultation writes the ruling record and the queue, so it runs only while no stage session is live, and the lead dispatches no stage beside it. Its agent type is never one `LIFECYCLE_KIT_STAGE_SESSION_TYPES` lists, whose stamp-before-write rule refuses a session that stamps nothing (§check-dispatch-entry). The batching threshold and the type are a slot rather than knobs, since no mechanism reads either; off is the default, and a consumer that wants fewer dispatches sets a count or an age. **Honest limit:** whether a stage session is live is a fact about a session no read of the tree settles (§The state machine), so the quiet-tree rule is prose like the lead's write bar.

### (3) The consult skill's dispatched mode {design-bearing}

**Not yet applied.** lifecycle-kit/templates/consult.md gains a paragraph after the ritual's numbered steps:

> **Dispatched by a lead, with no operator in the session**, take up consult-inbox items only. Re-class or discard an item alone, with the cause in the commit message. Escalate each item that needs a ruling or an operator direction to the lead, one Question / Options / Recommendation / Evidence block per item, all in one turn end; it stays in the inbox until an answer lands. Land a relayed answer in the class the relay names (lifecycle-kit/SPEC.md §The steering vocabulary).

lifecycle-kit/SPEC.md §templates/consult.md gains one paragraph:

> **A dispatched consultation decides only what needs no operator.** A lead dispatches one for inbox items (§templates/lead.md), and no operator sits in that session, so it re-classes and discards alone and escalates everything else through the lead. Re-classing and discarding are an item's routing, which any session may do with cause; a ruling is the operator's, and a relayed answer's class is §The steering vocabulary's open question.

### (4) The open question in the steering vocabulary {design-bearing}

**Not yet applied.** lifecycle-kit/SPEC.md §The steering vocabulary gains a paragraph after the one opening **Why five words and not one.**:

> **Open: is an operator answer relayed through a lead into a dispatched consultation a ruling?** Two definitions meet there. A ruling is taken in the consult skill, and a direction is an operator answer given in a lead session. Until the operator settles it, the answer lands as a direction, the class a relay cannot inflate. An item it cannot resolve without an entry on the ruling record stays in the consult inbox for a consultation the operator runs.

### (5) This repo's bindings {mechanical}

**Not yet applied.**

- `.claude/commands/lead.md` binds **consult-dispatch**: *the agent type is `consult-session` (`.claude/agents/consult-session.md`); the threshold is off.*
- `.claude/agents/consult-session.md`, new, `model: opus`: it invokes the consult skill in its dispatched mode on the items its prompt names; batches ruling-class escalations to the lead (`to: "main"`) as four-header blocks; journals per delegation-kit/templates/agent-execution.md, its **Findings you will act on are durable before you act on them** bullet; and dispatches no sibling session. It cites those surfaces rather than restating them.
- `scripts/delegation-config.knobs`: `DELEGATION_KIT_MUTATING_TYPES[] = consult-session`, since it commits to the shared tree; and `DELEGATION_KIT_STATUSLINE_INBOXES[] = C=.workflow/consult-items.md` and `DELEGATION_KIT_STATUSLINE_INBOXES[] = G=.workflow/gap-inbox.md`, each under a `# spec:` line citing delegation-kit/SPEC.md §The statusline arm. `LIFECYCLE_KIT_STAGE_SESSION_TYPES` is unchanged.

## Producers and consumers

Probe: `git grep -n` over the tracked tree for `STAGE_SESSION_TYPES`, `MUTATING_TYPES`, `counter group`, `queue_counts::emit` and `\*<` in lifecycle-kit/templates/lead.md.

- **`DELEGATION_KIT_STATUSLINE_INBOXES`** (delta 1). Producer: the consumer's knob file; this repo sets it (delta 5), so the path is live. Consumer: the statusline arm at every status-line fire, in process. Fields: the label is read by the render as the token's prefix; the path by the count. Roster-holding readers: the kit's static table and `--emit knob-roster`; the arm's `KNOBS` declaration, which the knob-file derivation, `check-reads-couples` and `check-gate-substrate-parity` read; `check-knob-citation`, satisfied by the SPEC bullet.
- **The consult dispatch** (deltas 2, 3). Producer: the lead, at a quiet point with the inbox at threshold. Consumer: the dispatched consultation, which reads the items its prompt names and the inbox. Its escalations reach the lead by the escalation protocol and the operator by the lead's relay.
- **The `consult-dispatch` slot** (delta 2). Reader: the lead at a quiet point. Roster-holding reader: `check-skill-binding`, which reds `.claude/commands/lead.md` until it binds the slot (delta 5), and reds every vendoring consumer's lead shim the same way.
- **The open question** (delta 4). Reader: a dispatched consultation landing a relayed answer, and the operator, who settles it in a consultation.
- **The consult-session type** (delta 5). Readers: the harness's agent registry; `check-agent-tier-explicit`, which reads its `model:` under `DELEGATION_KIT_REQUIRE_TIER = on`; the D4 roster, which admits its unisolated dispatch.

## Existing sections updated

Roster probe: the `git grep` above, plus `git grep -n` for `The statusline arm` and `§templates/consult.md`.

- `delegation-kit/SPEC.md` — §The statusline arm, §Layout and configuration (delta 1).
- `native/src/hook/statusline.rs`, `native/src/knobs/delegation_kit.rs` (delta 1).
- `lifecycle-kit/templates/lead.md` (delta 2); `lifecycle-kit/templates/consult.md` (delta 3).
- `lifecycle-kit/SPEC.md` — §templates/lead.md (delta 2); §templates/consult.md (delta 3); §The steering vocabulary (delta 4).
- `.claude/commands/lead.md`, `.claude/agents/consult-session.md`, `scripts/delegation-config.knobs` (delta 5).
- `docs/lifecycle-kit/SPEC.md`, `docs/delegation-kit/SPEC.md` — the generated on-site mirror, regenerated with `bash gate-sdk/bin/run-gates.sh --emit docs-mirror --write` (deltas 1, 2, 3 and 4; delta 5 touches no mirrored file).
- `.workflow/release-declarations.md` — Tightened gates gains a bullet led by `check-skill-binding`: lifecycle-kit/templates/lead.md gained the `consult-dispatch` slot, so a lead binding shim reds until it binds it; bind the agent type your lead dispatches a consultation as, and a threshold or off (delta 2). Behavior changes gains a bullet led by **delegation-kit/SPEC.md §The statusline arm**: an optional inbox counter group, `DELEGATION_KIT_STATUSLINE_INBOXES`, empty by default (delta 1).

## Retired spellings

- None — every delta adds or re-phrases prose and code; no name, path or token is retired.

## Definition of Done

- [ ] **Causal completeness** — every point of canon-kit/SPEC.md §The causal-completeness check holds for each new state, event, interface and obligation.
- [ ] **Instruction surfaces: instruction only** — replacement text for a template, agent definition or shim carries no grounds; a delta places them (the Content-tiering / SSOT rule).
- [ ] **Merged with no information lost** — each addition re-phrases the canonical-spec text it refines rather than appending to it; the merged spec reads as one document a reader who never saw the amendment can use alone.
- [ ] **Amendment deleted** — this file removed on merge; none remain for the component (`ls lifecycle-kit/SPEC-*.md`).
- [ ] **Removals propagated** — every name this change retired is declared in `## Retired spellings` above, and `check-amendment-retired-spelling` runs each declaration against the whole tracked tree, not against the specs alone.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed as debt tasks (a build-time causal gap is resolved that session, not deferred).
- [ ] **Battery green** — `bash gate-sdk/bin/build-native.sh`, the full battery, and the lifecycle-kit and delegation-kit fixture suites green on the landing commit. The entry moves to Done before the drain stage (`LIFECYCLE_KIT_DRAIN_STAGE`).
