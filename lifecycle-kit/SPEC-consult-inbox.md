# SPEC amendment: consult-inbox

Nothing queues work owed to the consult skill. Its only automatic trigger is the ruling-staleness probe, which the scope and close bindings read and the consult binding does not. So an operator-class reversal a stage finds, a threshold entry declined twice, a direction question misnamed as a ruling, or a vacant roadmap horizon is relayed live or lost. This amendment adds a committed inbox that only the consult skill drains, a capture arm that writes it, and a close-drain disposition that forwards a consult-class gap bullet there.

One queue entry pairs it: [consult-inbox](../TASK-QUEUE.md#consult-inbox). Two siblings depend on it, [consult-inbox-drain-trigger](../TASK-QUEUE.md#consult-inbox-drain-trigger) (its status cue and lead dispatch) and [roadmap-horizon-motion-unowned](../TASK-QUEUE.md#roadmap-horizon-motion-unowned) (its first producer beyond a live session). It merges before [lifecycle-kit-front-brevity](../TASK-QUEUE.md#lifecycle-kit-front-brevity) passes §The steering vocabulary.

**The rulings.**

- **A second inbox, not a second bullet class in the first.** The gap inbox's drain is close's, and close cannot rule a consult item, so a mixed inbox would stall close's drain or force close to judge what only the operator may. The two drains have different owners, so the surfaces are separate.
- **The gap inbox's grammar and capture contract, minus what is stage-shaped.** One `- <YYYY-MM-DD> — <prose>` bullet per item under a `# contract:` header. The arm keeps file-gap's repo-root anchor, argv-shape refusal, `--` escape, linked-worktree refusal and header seeding. It drops the live-slug advisory, because a recurrence is judged at close's drain and a consult item is no recurrence. It drops the cursor warning, because no stage drains this inbox.
- **Drained item by item, never truncated at a boundary.** A consultation takes up the items it reaches and removes each bullet in the commit that lands its disposition. An item the operator defers stays. No stage entry reads the inbox, so it refuses nothing and blocks nothing, and it is not in the boundary-truncate set: an owed consultation outlives the iteration that filed it.
- **Three dispositions**: ruled (landed where the consult binding's landing surfaces send a ruling), re-classed to the queue (filed as a gap, or a direct entry the operator directs), or discarded with cause in the landing commit's message.
- **Items are public-safe.** The inbox is tracked in a public tree. An item needing private context names a private-brief section rather than restating it.
- **Close forwards a consult-class gap bullet** (open for this stage on the entry). A gap bullet whose disposition is a ruling or an operator-only direction cannot be fixed, and promoting it parks a question as work that no build can land: the attested case sat in Deferred with *a ruling settles it* as its →fix failure until an operator answered it at a scope. So the drain gains →forward, tried after →fix and before →promote. The first stage's intake takes the same set.
- **No age escalation in this inbox** (open for this stage on the entry). The session-brief line prints the count and the oldest item's date, so age is visible at every session start. Escalating on age is a batching threshold, and that is the lead's dispatch trigger, consumer-bound and off by default (the drain-trigger sibling).
- **The session-brief line is the consumer copy's, not the context-kit template's.** context-kit's template stays uncoupled from lifecycle-kit, the precedent the budget-verdict line already set (delegation-kit/SPEC.md §Layout and configuration, its `agent-budget-guard` paragraph). This repo's copy prints one advisory line when the inbox holds a bullet.
- **The seam.** Kit mechanism: the surface, its knob, the arm, the drain contract, the forward disposition, the template step. Consumer config: the knob's value and the consult binding's entry-reading order. Private rule content: none enters, by the public-safe rule above.

**Refused.**

- **A boundary refusal on a non-empty inbox.** It would make an operator's absence block the machine, which is the opposite of an advisory surface. The operator direction ruled it advisory and never blocking a stage.
- **A `--to consult` mode on the file-gap arm.** One arm with two target surfaces and two stderr contracts is the two-output-grammar shape queue-kit/SPEC.md §The queue-counts arm refuses; a sibling arm reads plainer at every call site.
- **A close-surface declaration for the inbox.** Close's roster rows are surfaces close dispositions, and close cannot disposition this one. A declaration would put a row on close's sweep that close must skip every time.
- **A grammar gate.** check-gap-inbox-neutrality's assertion A was added because a malformed gap bullet reached a drain an iteration late. This inbox's one parser is a count of `- ` lines, which a malformed bullet still meets, and its drain reads prose. The arm stamps the grammar.

## What changes

### (1) The consult inbox section {design-bearing}

**Not yet applied.** lifecycle-kit/SPEC.md gains `## The consult inbox` between §The committed gap inbox and §The survey record:

> ## The consult inbox
>
> Work owed to the consult skill has a committed channel of its own: an operator-class finding a session cannot relay live, a threshold entry declined twice, a direction question misnamed as a ruling, and a signal a stage raises for the operator. Without one, such an item survives only if a live session relays it. The gap inbox cannot carry it: its drain is the closing stage's, and close cannot rule, so a mixed inbox would stall that drain.
>
> **The surface.** `.workflow/consult-inbox.md` (knob `LIFECYCLE_KIT_CONSULT_INBOX_FILE`, §Layout and configuration) is committed and append-only, with the gap inbox's grammar: a `# contract:` header, then one `- <YYYY-MM-DD> — <prose>` bullet per item. An item is public-safe; one needing private context names the private-brief section that holds it rather than restating it. It carries `merge=union` (§Multi-operator semantics), because an item filed on either side of a concurrent merge must survive.
>
> **The affordance.** `run-gates.sh --emit file-consult [--] "<item prose>"` appends one dated bullet, seeding the header when the inbox does not yet exist. Its argv, anchor and linked-worktree contract are §The committed gap inbox's affordance's, and stdout is the filed bullet. It resolves no live slug and reads no cursor: a consult item is not judged for recurrence, and no stage drains it. Its declared roster is `LIFECYCLE_KIT_CONSULT_INBOX_FILE` alone. The raw append stays a legal fallback.
>
> **Producers.** Any session in the main checkout that holds an item owed to the operator and cannot have it answered live: a stage session under no lead, a lead the operator deferred, a close forwarding a gap bullet (§The committed gap inbox), and a stage step that raises a consult-owed signal. A contrary finding against a ruling is filed here, never annotated on the record (§The steering vocabulary). The filing session commits its own bullet, under the gap inbox's index-freeness rule.
>
> **The drain is the consult skill's alone** (§templates/consult.md). A consultation takes up the items it reaches and gives each one disposition: **ruled**, landed where the consumer's landing surfaces send a ruling; **re-classed** to the queue, as a gap bullet or a direct entry the operator directs; or **discarded** with cause. The bullet is removed in the commit that lands its disposition, and that commit's message records the disposition. An item the operator defers stays. The file drains header-preservingly (gate-sdk/SPEC.md §The workflow directory).
>
> **It gates nothing and outlives iterations.** No stage entry reads it and the boundary truncates nothing in it, so a non-empty inbox refuses no transition. Its visibility is a count: a consumer's session brief or status line reads the number of `- ` lines, with the oldest bullet's date as its age. Escalation on count or age is the lead's dispatch trigger (§templates/lead.md), consumer-bound. It is on no close-surface row, since close cannot disposition it.
>
> **Honest limit.** Nothing gates the grammar; the arm stamps it, the counters read only the bullet prefix, and the drain reads prose. An item nobody ever consults stays owed and visible, which is the advisory posture chosen, not a silent loss.

### (2) The knob {mechanical}

**Not yet applied.** lifecycle-kit/SPEC.md §Layout and configuration gains, after the `LIFECYCLE_KIT_GAP_INBOX_FILE` bullet:

> - `LIFECYCLE_KIT_CONSULT_INBOX_FILE` — the committed consult inbox (§The consult inbox); default `${GATE_SDK_WORKFLOW_DIR:-.workflow}/consult-inbox.md`, written by the `--emit-file-consult` arm, its `merge=union` attribute verified by `check-merge-attrs`, drained by the consult skill. A kit default rather than empty on the `LIFECYCLE_KIT_GAP_INBOX_FILE` posture: the kit's own arm mints the artifact.

`native/src/knobs/lifecycle_kit.rs` gains the row, `in_workflow_dir(resolve, "consult-inbox.md")` beside the gap inbox's.

### (3) Contrary evidence against a ruling is filed {mechanical}

**Not yet applied.** lifecycle-kit/SPEC.md §The steering vocabulary, the **Ruling** bullet: the clause *a session holding contrary evidence escalates it to the next consultation and neither annotates it, re-verifies it, nor works around it* is replaced by:

> a session holding contrary evidence files it to the consult inbox (§The consult inbox) and neither annotates the ruling, re-verifies it, nor works around it.

### (4) The drain gains →forward {design-bearing}

**Not yet applied.** Three passages, one disposition set.

lifecycle-kit/SPEC.md §The committed gap inbox, the paragraph opening **The drain's dispositions are ordered, and promotion is last**: its first sentence pair is replaced by:

> **The drain's dispositions are ordered, and promotion is last, on a measured drain rather than on a preference.** The disposition set is fix, forward, promote, discard, tried in that order per bullet, and a promotion states in the close commit message why neither fix nor forward took it. **Forward** re-files a bullet whose disposition is a ruling or an operator-only direction to the consult inbox (§The consult inbox) with `--emit file-consult`, its prose carried and its original date named in it: a question only the operator can settle is not work, and promoting it parks it as an entry no build can land.

The rest of that paragraph stands.

lifecycle-kit/templates/stages/close.md step 2: the sentence opening *The disposition set, **tried in this order*** gains →forward between →fix and →promote:

> →forward (a bullet only the operator can settle — a ruling, or a direction no stage may give: re-file it with `--emit file-consult`, carrying its prose and its date),

and the sentence *A →promote states in the commit message why →fix failed* becomes *A →promote states in the commit message why →fix and →forward failed*.

lifecycle-kit/templates/stages/scope.md, the second step: *promoted to a queue entry after the owner lookup (…), fixed inline this session, or discarded with cause in the commit message* becomes *promoted to a queue entry after the owner lookup (…), fixed inline this session, forwarded to the consult inbox (`--emit file-consult`), or discarded with cause in the commit message*.

### (5) The consult skill drains the inbox {design-bearing}

**Not yet applied.** lifecycle-kit/templates/consult.md: the exit condition gains a clause, after *every ruling the operator closed in the session has landed in a governed surface,*:

> every consult-inbox item the session took up is dispositioned,

and the ritual gains a step after step 1, the later steps renumbered:

> 2. **Drain the consult inbox (`LIFECYCLE_KIT_CONSULT_INBOX_FILE`) as you reach its items.** Give each item you take up one disposition: ruled, landed as step 3 lands a ruling; re-classed to the queue, filed with `--emit file-gap` or entered directly where the operator directs; or discarded, with the cause in the commit message. Remove its bullet in the commit that lands the disposition, and name the disposition in that message. An item the operator defers stays in the inbox.

lifecycle-kit/SPEC.md §templates/consult.md, the paragraph opening **What the template owns** gains one sentence after its first:

> The landing contract reaches the consult inbox (§The consult inbox): an item the session takes up leaves the inbox only by a disposition landed in the same commit, so the inbox holds exactly what is still owed.

### (6) The capture arm {design-bearing}

**Not yet applied.** `native/src/emit/file_consult.rs` implements `--emit-file-consult` to §The consult inbox's affordance (delta 1), and `native/src/emit/mod.rs` registers it in the arm table beside `--emit-file-gap` with `Grammar::Parsed` and its one-knob roster. It reuses `file_survey::positionals`, `anchored`, `refuse_in_linked_worktree` and `append`, and seeds its own contract header:

> `# contract: lifecycle-kit/SPEC.md §The consult inbox — append-only capture of items owed to the consult skill, consult-drained; one bullet per item below.`

Unit tests: the one bullet shape, arity misuse refused, a leading `-` refused and `--` honoured. `lifecycle-kit/gate-tests/capture-linked-worktree.test.sh` adds the arm to the two it runs from a real linked worktree, asserting the refusal and the hand-back steer.

### (7) The inbox joins the union set and the design-ahead valve {mechanical}

**Not yet applied.**

- `stages::union_set` returns the consult inbox beside the gap inbox, so `--install-lifecycle` emits its `merge=union` line and `check-merge-attrs` holds it forward-only. The merge-attrs `help:` line names `LIFECYCLE_KIT_CONSULT_INBOX_FILE` beside the gap inbox's knob, and `native/src/emit/install_lifecycle.rs`' knob roster gains it. This repo's `.gitattributes` block is regenerated by `--install-lifecycle`.
- lifecycle-kit/SPEC.md §Multi-operator semantics: *The kit owns exactly one `union`-driver surface — the committed gap inbox (§The committed gap inbox), whose append-only bullets must survive a concurrent merge rather than supersede —* becomes *The kit owns two `union`-driver surfaces — the committed gap inbox and the consult inbox (§The consult inbox), whose append-only bullets must survive a concurrent merge rather than supersede —*.
- lifecycle-kit/SPEC.md §check-merge-attrs: *the derived union set (`stages::union_set` — the gap inbox)* becomes *(`stages::union_set` — the gap and consult inboxes)*, and *an empty union set (it always owns at least its gap inbox)* stands.
- lifecycle-kit/SPEC.md §bin/install-lifecycle.sh, the merge-attribute step: *one `merge=union` line per union member (the gap inbox, git-native)* becomes *(the two inboxes, git-native)*.
- canon-kit/SPEC.md §Layout and configuration, `check-kit-ref-liveness`'s valved surfaces: *the two design-ahead records, the queue and the gap inbox (`LIFECYCLE_KIT_GAP_INBOX_FILE`), each read by its knob. Both name knobs and paths not yet minted, and the inbox cannot hold one past the close that truncates it;* becomes *the three design-ahead records, the queue and the two inboxes (`LIFECYCLE_KIT_GAP_INBOX_FILE`, `LIFECYCLE_KIT_CONSULT_INBOX_FILE`), each read by its knob. Each names knobs and paths not yet minted; the gap inbox cannot hold one past the close that truncates it, and a consult item holds one until a consultation disposes of it;* and `native/src/gates/kit_ref_liveness.rs` valves the consult inbox by its knob's resolved path.
- lifecycle-kit/SPEC.md §check-dispatch-entry, the skip: *A commit whose staged path set is exactly the gap inbox (`LIFECYCLE_KIT_GAP_INBOX_FILE`), which is the lead's one sanctioned commit* becomes *A commit whose staged path set is exactly one inbox, the gap inbox or the consult inbox (`LIFECYCLE_KIT_GAP_INBOX_FILE`, `LIFECYCLE_KIT_CONSULT_INBOX_FILE`), which is a lead's sanctioned capture commit*; `native/src/gates/dispatch_entry.rs` reads both knobs and `gate-tests/check-dispatch-entry.test.sh` adds the consult-inbox skip case.
- gate-sdk/SPEC.md §The workflow directory: *the two tracked capture arms refuse there instead of routing* becomes *the three tracked capture arms refuse there instead of routing*; and the draining-member list *`gap-inbox.md` at close* becomes *`gap-inbox.md` at close, `consult-inbox.md` item by item at a consultation*.

### (8) The lead files what the operator defers {mechanical}

**Not yet applied.** lifecycle-kit/templates/lead.md:

- §The escalation protocol, the paragraph opening **One class the lead never rules, under either posture.**: after *the lead relays it, however well-grounded the escalating session's finding and however urgent the fix.* insert *Where the operator defers it, file it with `--emit file-consult` so it reaches the next consultation rather than your transcript.*
- §Stamps are authoritative, the paragraph opening **While a dispatched stage session is live**: *A capture you make, a gap bullet or a survey block, goes to your journal.* becomes *A capture you make, a gap bullet, a consult item or a survey block, goes to your journal.*
- The paragraph opening **File and commit your captures between dispatches.**: *(`--emit file-gap`, `--emit file-survey`)* becomes *(`--emit file-gap`, `--emit file-consult`, `--emit file-survey`)*, and *Neither capture arm makes a `git` call* becomes *No capture arm makes a `git` call*.

### (9) This repo's bindings and resident line {mechanical}

**Not yet applied.**

- `.claude/commands/consult.md`, **entry-reading**: *three surfaces, in this order: `TRAJECTORY.md` (…), `TASK-QUEUE.md`* becomes *four surfaces, in this order: `TRAJECTORY.md` (…), `.workflow/consult-inbox.md` (the items owed to this session, drained per the template's step 2), `TASK-QUEUE.md`*.
- `CLAUDE.md` §Housekeeping, the `TRAJECTORY.md` bullet: *a ruling is closed: escalate to `/consult`, never reverse, annotate or re-verify.* becomes *a ruling is closed: file contrary evidence with `--emit file-consult` for `/consult`, never reverse, annotate or re-verify.* No new resident line.
- `TRAJECTORY.md`, **Three acts**: *contrary evidence is escalated, never annotated in place* becomes *contrary evidence is filed to the consult inbox, never annotated in place*. This edits the record's contract paragraph, not a ruling.
- `scripts/session-context.sh` (the consumer copy) gains a block after the budget-verdict line, under `# spec: lifecycle-kit/SPEC.md §The consult inbox`: resolve `CONSULT_INBOX="${LIFECYCLE_KIT_CONSULT_INBOX_FILE:-${GATE_SDK_WORKFLOW_DIR:-.workflow}/consult-inbox.md}"`, and where it holds at least one `- ` line print `Consult inbox: <n> item(s) owed to /consult, oldest <date>.` and a blank line; silent otherwise, and guarded so it never fails the session.

## Producers and consumers

Probe for each roster: `git grep -n` over the tracked tree for `file-gap`, `file_gap`, `GAP_INBOX_FILE`, `file-survey`, `union_set`, `gap inbox` and `escalate to`.

- **The consult inbox** (deltas 1, 2).
  - Producer: `--emit file-consult` (delta 6), live under the kit default with no config set; the close drain's →forward (delta 4); a lead's deferred operator-class escalation (delta 8); a session holding contrary evidence against a ruling (deltas 3, 9); the roadmap signal once its sibling lands.
  - Consumers: the consult skill's drain step (delta 5), reached at every consultation through the binding's entry-reading (delta 9); the session brief's count line (delta 9); git's union driver at a merge and `check-merge-attrs` at pre-commit (delta 7); `check-kit-ref-liveness` as a valved surface (delta 7); the drain-trigger sibling's status-line counter and lead dispatch.
  - Fields: the date is read by the count line as the oldest item's age and by the consulting session as the item's age; the prose is the disposition body the consultation reads. No third field.
- **`LIFECYCLE_KIT_CONSULT_INBOX_FILE`** (delta 2). Readers: the arm, `stages::union_set`, `install_lifecycle`, `kit_ref_liveness`, `dispatch_entry`, the consult template by name, the consumer brief. Roster-holding readers: the kit's static table (`--emit knob-roster`), and `check-knob-citation`, which the SPEC bullet satisfies.
- **`--emit-file-consult`** (delta 6). Roster-holding readers: the arm table; the front-end's arm list, derived from it; `capture-linked-worktree.test.sh`, which names the tracked capture arms it runs; lifecycle-kit/README.md §Use's arm block.
- **→forward** (delta 4). Producer: the close drain and the first stage's intake. Consumer: the consult inbox. Its record is the close commit message beside the other dispositions.
- **Red conditions.** `check-merge-attrs` reds a union member missing its `merge=union` line, so delta 7's `.gitattributes` regeneration lands in the same commit as `union_set`. `check-dispatch-entry`'s skip widens, which removes reds only.

## Existing sections updated

Roster probe: the `git grep` above, plus `git grep -n` for `two tracked capture arms`, `design-ahead records`, `exactly one \`union\``, `the gap inbox, git-native` and `Neither capture arm`.

- `lifecycle-kit/SPEC.md` — §The consult inbox, new (delta 1); §Layout and configuration (delta 2); §The steering vocabulary (delta 3); §The committed gap inbox (delta 4); §templates/consult.md (delta 5); §Multi-operator semantics, §check-merge-attrs, §bin/install-lifecycle.sh, §check-dispatch-entry (delta 7).
- `lifecycle-kit/templates/stages/close.md`, `lifecycle-kit/templates/stages/scope.md` (delta 4).
- `lifecycle-kit/templates/consult.md` (delta 5).
- `lifecycle-kit/templates/lead.md` (delta 8).
- `native/src/emit/file_consult.rs`, `native/src/emit/mod.rs` (delta 6); `native/src/knobs/lifecycle_kit.rs` (delta 2); `native/src/stages.rs`, `native/src/gates/merge_attrs.rs`, `native/src/emit/install_lifecycle.rs`, `native/src/gates/kit_ref_liveness.rs`, `native/src/gates/dispatch_entry.rs` (delta 7).
- `lifecycle-kit/gate-tests/capture-linked-worktree.test.sh` (delta 6); `lifecycle-kit/gate-tests/check-dispatch-entry.test.sh` (delta 7).
- `canon-kit/SPEC.md` — §Layout and configuration (delta 7).
- `gate-sdk/SPEC.md` — §The workflow directory (delta 7).
- `lifecycle-kit/README.md` — §Use gains `"$gates" --emit file-consult "<item>"   # route an item owed to the consult skill to its inbox`, and install step 4's *a `merge=union` line for the committed gap inbox* becomes *a `merge=union` line for each committed inbox* (deltas 6 and 7).
- `.gitattributes` (delta 7); `.claude/commands/consult.md`, `CLAUDE.md`, `TRAJECTORY.md`, `scripts/session-context.sh` (delta 9).
- `docs/lifecycle-kit/SPEC.md`, `docs/lifecycle-kit/README.md`, `docs/canon-kit/SPEC.md`, `docs/gate-sdk/SPEC.md` — the generated on-site mirror, regenerated with `bash gate-sdk/bin/run-gates.sh --emit docs-mirror --write` (all deltas).
- `.workflow/release-declarations.md` — Behavior changes gains a bullet led by **lifecycle-kit/SPEC.md §The consult inbox**: a committed consult inbox (`LIFECYCLE_KIT_CONSULT_INBOX_FILE`, `--emit file-consult`) drained by the consult skill, a `merge=union` line `--install-lifecycle` writes, and a →forward disposition in close's gap drain; re-run `--install-lifecycle` to add the attribute (deltas 1, 4, 6 and 7).

## Retired spellings

- None — every delta adds or re-phrases prose and code; no name, path or token is retired.

## Definition of Done

- [ ] **Causal completeness** — every point of canon-kit/SPEC.md §The causal-completeness check holds for each new state, event, interface and obligation.
- [ ] **Instruction surfaces: instruction only** — replacement text for a template, agent definition or shim carries no grounds; a delta places them (the Content-tiering / SSOT rule).
- [ ] **Merged with no information lost** — each addition re-phrases the canonical-spec text it refines rather than appending to it; the merged spec reads as one document a reader who never saw the amendment can use alone.
- [ ] **Amendment deleted** — this file removed on merge; none remain for the component (`ls lifecycle-kit/SPEC-*.md`).
- [ ] **Removals propagated** — every name this change retired is declared in `## Retired spellings` above, and `check-amendment-retired-spelling` runs each declaration against the whole tracked tree, not against the specs alone.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed as debt tasks (a build-time causal gap is resolved that session, not deferred).
- [ ] **Battery green** — `bash gate-sdk/bin/build-native.sh`, the full battery, and the lifecycle-kit, canon-kit and gate-sdk fixture suites green on the landing commit. The entry moves to Done before the drain stage (`LIFECYCLE_KIT_DRAIN_STAGE`).
