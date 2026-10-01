# SPEC amendment: consult-intake

**The consult inbox takes two classes, a ruling to reconsider and a request for a judgment-tier recommendation. Every other item owed the operator is asked live or carried by the gap inbox to the next scope.** lifecycle-kit/SPEC.md §The consult inbox admits four classes: an operator-class finding a session cannot relay live, a threshold entry declined twice, a direction question misnamed as a ruling, and a signal a stage raises for the operator. The operator's model of consult is narrower (operator direction, 2026-09-29, lead-relayed): consult is for reconsidering a ruling and for an expert opinion from a superior model on strategic direction. The catch-all classes pulled in questions the operator had already answered as a direction, and each misfiled item cost a consultation to route it back out. The operator's addendum (operator direction, 2026-09-30, lead session) adds what consult does with a request:

- a request expecting a reply gets one;
- one expecting a tracked change makes it through the ordinary workflow;
- a ruling reaches the ruling record only where that workflow would otherwise challenge or reject the direction.

The lead weighed a caution with that addendum: a change consult lands carries its provenance as consult advice, marked operator-accepted or not. At this unit's authoring the operator chose, with that marking shown, to keep the lead-dispatched consultation for requests for advice under a sixth steering term (operator direction, 2026-10-01, lead session).

The change spans lifecycle-kit's SPEC and templates and this repo's consultation agent definition, so this amendment sits at the repository root.

**Read at authoring:**

- **The producers**, by `git grep -n -i "file-consult\|consult inbox\|consult item\|consult-owed" -- ':!docs'`. Four template sites file to the inbox:
  - `lead.md`'s operator-class paragraph files a ruling reversal the operator deferred;
  - `close.md` step 2's →forward re-files a gap bullet only the operator can settle;
  - `close.md` step 5's roadmap-motion read files the slugs that left the projection and a vacant first horizon;
  - `scope.md`'s second step forwards a carried gap bullet.

  `lead.md`'s capture paragraph and the kit README's example name the arm generically. The threshold-declined class has no filing site, since scope's escalation already routes a third decline to the operator (lifecycle-kit/SPEC.md §templates/stages/, *The scope template*). The misnamed-question class is classification prose alone (§The steering vocabulary).
- **No gate holds the producer list.** The `--emit file-consult` arm records a date and prose and no class field (`native/src/emit/file_consult.rs`). The inbox's readers are counters: the session brief's count line, the statusline, and the lead's dispatch threshold. So narrowing is a SPEC and template change, and the arm and its contract header stand.
- **The other classes already have a destination.** lifecycle-kit/templates/lead.md §Mid-iteration intake routes a question to the operator through a live lead and work to the gap inbox, and its grounds anticipated this narrowing. A stage session run with no lead surfaces to the operator directly (build.md's question triage). What lacked a home was a question the operator defers, or one raised where no live channel reaches. This amendment carries it on the gap inbox to the next first-stage intake, whose escalation reaches the operator in every posture.
- **consult.md** lands every closed ruling and has no path for advice: every step ends in a ruling, a re-class or a discard. It already checks the judgment tier at entry (*First step — verify your tier*).
- **The ruling record** (`TRAJECTORY.md`) holds no ruling today. CLAUDE.md's always-loaded line routes contrary evidence to `--emit file-consult`, which stays a consult class.

## What changes

### (1) The consult inbox admits two classes

lifecycle-kit/SPEC.md §The consult inbox: the opening paragraph and the **Producers.** paragraph are rewritten. {design-bearing} {user-facing: operator direction 2026-09-29, consult is for a ruling to reconsider and a superior model's opinion} **Not yet applied.**

The opening paragraph becomes:

> Work owed to the consult skill has a committed channel of its own, and two classes enter it. **A ruling to reconsider** is evidence contrary to a recorded ruling or objective, filed rather than annotated on the record (§The steering vocabulary), or a reversal the operator deferred. **A request for advice** is a strategic question, architecture say, on which the operator wants a judgment-tier recommendation, filed by the operator or at the operator's request. Without the channel such an item survives only if a live session relays it. The gap inbox cannot carry it: its drain is the closing stage's, which can neither rule nor advise at the tier consult declares, so a mixed inbox would stall that drain or force close to judge what only the operator may.

The **Producers.** paragraph becomes two:

> **Producers.** A session in the main checkout holding an item of either class: contrary evidence against a ruling, a lead whose operator deferred a reversal (§templates/lead.md), or any session the operator asks to queue a question for advice. A session never files its own question as a request for advice. The filing session commits its own bullet, under the gap inbox's index-freeness rule.
>
> **Everything else owed the operator is asked live.** A direction question, an entry declined twice and a stage's signal for the operator go to the operator through the lead where one is live, and directly where the operator runs the session. One the operator defers, or one no live channel reaches, is filed to the gap inbox (§The committed gap inbox), and the next first-stage intake puts it in its escalation, the one stage escalation that reaches the operator in every posture.

In the paragraph **Ruling on an item is the consult skill's alone**, "**ruled**, landed where the consumer's landing surfaces send a ruling;" is followed by "**answered**, a request for advice given its recommendation (§templates/consult.md);".

### (2) The gap inbox carries a deferred operator question

lifecycle-kit/SPEC.md §The committed gap inbox: three passages are re-phrased. {design-bearing} **Not yet applied.**

- In the first paragraph, "a *work-shaped* finding (a gap, a task, a defect) is backlog and routes here" becomes "a *work-shaped* finding (a gap, a task, a defect) is backlog and routes here, and so does an operator question nobody could ask live (§The consult inbox), which this channel carries to the next intake".
- In **The drain's dispositions are ordered**, "**Forward** re-files a bullet whose disposition is a ruling or an operator-only direction to the consult inbox (§The consult inbox) with `--emit file-consult`, carrying its prose and naming its original date; a question only the operator can settle is not work a build can land." becomes "**Forward** re-files a bullet of a consult class (§The consult inbox) with `--emit file-consult`, carrying its prose and naming its original date. A bullet asking the operator for a direction is **asked** instead, put to the operator through the live channel, or re-filed after the drain's truncation for the next intake to ask, since a question only the operator can settle is not work a build can land."
- In **Producers and consumers.**, the close drain's disposition list "fixed inline that session, forwarded to the consult inbox, promoted to a deferred entry, or discarded with cause" gains "asked of the operator," after "forwarded to the consult inbox,".

### (3) The stage and lead templates route by class

{mechanical} **Not yet applied.**

- `lifecycle-kit/templates/stages/close.md`, step 2: "→forward (a bullet only the operator can settle — a ruling, or a direction no stage may give: re-file it with `--emit file-consult`, carrying its prose and its date)" becomes "→forward (a bullet of a consult class, a ruling to reconsider or a question the operator asked to have advised on: re-file it with `--emit file-consult`, carrying its prose and its date; a bullet asking the operator for a direction no stage may give is asked instead, through the lead where one is live and directly where the operator runs this session, and one the operator defers is re-filed with `--emit file-gap` after this step's truncation, for the next intake)".
- `close.md`, step 5: "File one consult-owed item with `--emit file-consult` naming every slug the first output lists and the second does not, and one when the first configured horizon holds no bullet now, unless an item already in the consult inbox names it." becomes "Ask the operator, as step 2 asks a direction question, about every slug the first output lists and the second does not, and about a first configured horizon holding no bullet now, unless the gap inbox already carries that question." In the same step, "Re-tag nothing else: a consultation sets new direction." becomes "Re-tag nothing else: the operator sets new direction."
- `lifecycle-kit/templates/stages/scope.md`, second step: "forwarded to the consult inbox (`--emit file-consult`)" becomes "forwarded to the consult inbox (`--emit file-consult`) when it is of a consult class, asked in this session's escalation when it asks the operator for a direction".
- `lifecycle-kit/templates/lead.md` §The escalation protocol: the operator-class paragraph keeps its consult filing for a deferred reversal and gains "A direction question the operator defers you file with `--emit file-gap`, so the next scope asks it again." The consult-dispatch paragraph's "Relay each ruling-class escalation to the operator and relay the answer back, stating its class." becomes "Relay each ruling-class escalation and each recommendation it sends to the operator, the recommendation as advice, and relay the answer back, stating its class: an accepted recommendation returns as a direction."

`roadmap-horizon-lag-detector`, promoted this iteration, adds a step-5 sentence routing its rows "with" the motion read's findings, naming no route. So this delta's rewrite of the motion read carries those rows whichever unit lands first.

### (4) The consult template answers advice and lands changes through the ordinary workflow

`lifecycle-kit/templates/consult.md` is re-phrased in four places. {design-bearing} {user-facing: operator direction 2026-09-30, a reply for a reply, the ordinary workflow for a change, the ruling record only where that workflow would reject the direction} **Not yet applied.**

- The opening's "an operator strategy session whose conclusions leave the transcript" becomes "a session that reconsiders a ruling with the operator or gives a judgment-tier recommendation on strategic direction, and whose conclusions leave the transcript". Its exit condition gains "every request for advice it took up is answered," after "every consult-inbox item the session took up is dispositioned,".
- Step 2's disposition list gains "answered, as step 3 answers a request for advice;" after "ruled, landed as step 3 lands a ruling;".
- Step 3 becomes:

  > 3. **Land each ruling and answer each request for advice the moment it closes, never at exit.** A request expecting a reply gets the reply. One expecting a tracked change lands it through the ordinary workflow, the deferred entry the operator names added, updated or removed or a gap bullet, stated as `operator direction, <date>, on consult advice` where the operator accepted it and `consult advice, <date>` where not. A ruling lands on the ruling record only where that workflow would otherwise challenge or reject the direction, an owner doc still stating what the direction reverses. The landing target is chosen by what the ruling is about, from *<landing-surfaces: …>* (the slot unchanged). Landing at exit instead is the failure mode itself, one step later: a session that batches to the end loses everything together when it is interrupted.

- The dispatched-mode paragraph becomes:

  > **Dispatched by a lead, with no operator in the session**, take up consult-inbox items only. Re-class or discard a misfiled item alone, with the cause in the commit message. Answer a request for advice alone: land the change your recommendation calls for as step 3 says, marked `consult advice, <date>`, remove its bullet, and send the recommendation to the lead so the operator may accept it; an acceptance the lead relays back re-marks the change `operator direction, <date>, on consult advice`. Escalate each ruling to reconsider to the lead too, one Question / Options / Recommendation / Evidence block per item, all in one turn end. It stays in the inbox, since a relayed answer lands in the class the relay names, never as a ruling (lifecycle-kit/SPEC.md §The steering vocabulary), so it waits for the operator's own consultation.

### (5) The steering vocabulary names advice

lifecycle-kit/SPEC.md §The steering vocabulary. {design-bearing} {user-facing: operator direction 2026-10-01, lead session, the Advice term with the marking the lead's 2026-09-30 caution proposed} **Not yet applied.**

- "Five terms name what an operator or a lead says to the work" becomes "Six terms name what an operator, a lead or a consultation says to the work".
- In the **Direction** bullet, "given in a lead or stage session" becomes "given in a lead, stage or consult session".
- A bullet is added after **Decision**:

  > - **Advice** — a judgment-tier consultation's recommendation. Accepted by the operator, it is a direction and lands as one, stated as `operator direction, <date>, on consult advice`. Unaccepted, it lands only on the work surface it concerns, stated as `consult advice, <date>`, and binds like a decision: a later scoping or authoring stage may revise it. It never enters the ruling record.

- In the closing paragraph, "a direction the operator's in session, a decision the relaying party's own" gains ", advice the consultation's".

### (6) The template contracts carry the grounds

lifecycle-kit/SPEC.md: three passages. {design-bearing} **Not yet applied.**

- §templates/consult.md, **What the template owns**, gains as its closing sentences: "**An answer is not a ruling.** A request for advice is answered in the session and only the change it leads to lands, through the workflow that lands any change, so the ruling record stays an override ledger and never fills with recommendations nothing needed protecting from."
- §templates/consult.md, the paragraph **A dispatched consultation decides only what needs no operator.** becomes:

  > **A dispatched consultation decides only what needs no operator, and advice needs none.** A lead dispatches one for inbox items (§templates/lead.md), and no operator sits in that session. So it re-classes and discards alone, and answers a request for advice alone, because a judgment-tier recommendation is what the request asks for and the operator's acceptance can follow it. What it lands carries the advice mark, so a later session tells model advice from operator direction. A ruling to reconsider waits for the operator: a relayed answer lands as a direction (§The steering vocabulary), never as a ruling.

- §templates/stages/, *The close template*, the paragraph **The roadmap-motion read**: "It files each finding to the consult inbox (§The consult inbox), since new direction is the operator's, and reads the inbox before filing, so a standing vacancy is one owed question." becomes "It asks the operator each finding, as the drain asks a direction question (§The consult inbox), since new direction is the operator's, and reads the gap inbox first, so a standing vacancy is one owed question."

### (7) This repo's consultation type and the kit README follow

{mechanical} **Not yet applied.**

- `.claude/agents/consult-session.md`'s description, "re-classes or discards alone, and batches every ruling-class item back to the lead", becomes "re-classes or discards alone, answers requests for advice, and batches every ruling-class item and recommendation back to the lead".
- `lifecycle-kit/README.md`'s example comment, "route an item owed to the consult skill to its inbox", becomes "route a ruling to reconsider or a request for advice to the consult inbox".
- `docs/lifecycle-kit/SPEC.md` and `docs/lifecycle-kit/README.md` are regenerated with `bash gate-sdk/bin/run-gates.sh --emit docs-mirror --write` in the commits landing deltas 1, 2, 5, 6 and this README line.

## Producers and consumers

- **The narrowed intake (delta 1).** Producers of class (a): a session holding contrary evidence (CLAUDE.md's always-loaded line, unchanged) and a lead whose operator deferred a reversal (lead.md, unchanged). Producer of class (b): a session the operator asks. Consumers: the consult skill's drain (operator-run or dispatched), and the inbox's three counters, which read the bullet prefix only and are unchanged.
- **The asked question (deltas 2 and 3).** Producers: close's drain, close's roadmap-motion read, a lead whose operator deferred a direction question. Its carrier is the existing `--emit file-gap` arm, so its bullet keeps the two fields `check-gap-inbox-neutrality` holds. Consumers: the live lead (relay), the operator directly, and the next first-stage intake, whose disposition set gains "asked". No new interface. The boundary check admits a post-close bullet as it does any other (§The committed gap inbox).
- **The Advice mark (deltas 4 and 5).** Producer: a consultation answering a request for advice, operator-run or dispatched; this repo dispatches through `consult-session`, so the producer is deployed. The mark's two forms each have readers: a later scoping or authoring stage reads `consult advice, <date>` as revisable, and every session reads `operator direction, <date>, on consult advice` as a direction. No gate parses the mark: the inline class statement is prose by queue-kit/SPEC.md §The tag algebra, which states a direction's class beside its content and refuses a tag for it.
- **Readers whose verdict moves.** None. No gate counts producers or parses classes. `check-skill-binding` holds `consult.md`'s and `lead.md`'s slot sets, which no delta changes. `capture-linked-worktree.test.sh` asserts the arm's contract header, which stands.
- **Narrowing.** The producer set narrows; no reader asserts a count or minimum over consult items, and the inbox is empty today.

## Existing sections updated

Roster produced by `git grep -n -i "file-consult\|consult inbox\|consult item\|consult-owed\|Five terms\|dispatched consultation" -- ':!TASK-QUEUE.md' ':!docs' ':!SPEC-*.md'`, with each hit read.

- `lifecycle-kit/SPEC.md` — §The consult inbox (delta 1), §The committed gap inbox (delta 2), §The steering vocabulary (delta 5), §templates/consult.md and §templates/stages/ *The close template* (delta 6).
- `lifecycle-kit/templates/stages/close.md` — steps 2 and 5 (delta 3).
- `lifecycle-kit/templates/stages/scope.md` — the second step (delta 3).
- `lifecycle-kit/templates/lead.md` — §The escalation protocol, the operator-class and consult-dispatch paragraphs (delta 3).
- `lifecycle-kit/templates/consult.md` — the opening, steps 2 and 3 and the dispatched-mode paragraph (delta 4).
- `.claude/agents/consult-session.md` — the description (delta 7).
- `lifecycle-kit/README.md` — the `--emit file-consult` example comment (delta 7).
- `docs/lifecycle-kit/SPEC.md`, `docs/lifecycle-kit/README.md` — the regenerated mirrors (deltas 1, 2, 5, 6 and 7).

The other hits need no edit. `CLAUDE.md` and `TRAJECTORY.md` route contrary evidence to the arm, which is class (a). §The steering vocabulary's ruling bullet and its paragraph on a relayed answer into a dispatched consultation stay true. The arm, its contract header and its test, the merge-attribute and union-set sites, the dispatch-entry skip and `scripts/session-context.sh`'s counter read the inbox without its classes. `.claude/commands/lead.md`'s `consult-dispatch` binding keeps its type and threshold, and `canon-kit/SPEC.md` names the inbox among the design-ahead records.

## Retired spellings

- None — close step 5's `consult-owed item` wording is re-phrased in place by delta 3, and its one other survivor is a dated release note that stays as history.

## Definition of Done

- [ ] **Causal completeness** — every point of canon-kit/SPEC.md §The causal-completeness check holds for each new state, event, interface and obligation.
- [ ] **Instruction surfaces: instruction only** — replacement text for a template, agent definition or shim carries no grounds; a delta places them (the Content-tiering / SSOT rule).
- [ ] **Merged with no information lost** — each addition re-phrases the canonical-spec text it refines rather than appending to it; the merged spec reads as one document a reader who never saw the amendment can use alone.
- [ ] **Amendment deleted** — this file removed on merge; no root-level amendment of this unit remains (`ls SPEC-*.md`).
- [ ] **Removals propagated** — every name this change retired is declared in `## Retired spellings` above, and `check-amendment-retired-spelling` runs each declaration against the whole tracked tree, not against the specs alone.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed as debt tasks (a build-time causal gap is resolved that session, not deferred).
- [ ] **The entry moves** — `consult-intake-narrowing` moves to Done with `--queue done` in the commit deleting this file, at its build batch, before the drain stage.
