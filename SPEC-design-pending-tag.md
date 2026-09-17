# SPEC amendment: design-pending-tag

**The `[design-pending]` tag restates the section that holds its entry, and it goes.**
Every entry in a design-pending section carries the tag, and no entry outside one may.
That coupling is total, so the tag carries nothing that section membership does not
already carry. queue-kit's icebox tier already refuses an `[icebox]` tag on this exact
ground: "a tag restating its own section is the two-sources defect". The same string
also names two things. On a lead line it is a section marker with no information. In
a body, `Why [design-pending]` is a field whose job is semantic. Sessions have misread
the first as the second.

**Operator direction, 2026-09-17, lead-relayed:** remove the tag and rely on section
membership, as the icebox tier does. A rename and keep-and-document were weighed and
not chosen. This amendment carries that direction out. It does not reopen it.

**The checksum the tag was defended as is already held elsewhere.** canon-kit's
§The amendment lifecycle calls the tag "a checksum on the promotion move". That role
is covered in both directions by tags that carry information:

- **Feature moves** are guarded by `[spec:]`. A feature-section entry without it reds,
  and a design-pending-section entry with it reds.
- **Deferred ↔ active moves of any entry** are guarded by the board tags, wherever
  `check-deferred-board-tags` is registered. Probed 2026-09-17 at spec on a scratch
  queue. An active entry carrying `[cost:]`/`[surface:]` reds under assertion B. A
  deferred entry lacking them reds under assertion A. So the accidental-demotion
  measurement the queue entry owed is moot: that direction already has a checksum.

**What stays as it is.** The **design-pending section set** (deferred plus a configured
icebox), its name, and every knob that configures it. The `[spec:]` tag and its
pairing arms. The board tags and their gate. The unbracketed word `design-pending` as
the name of that state and that section set.

## What changes

### (1) canon-kit: design-pending is a section state with no tag

§The amendment lifecycle drops the tag from the design-pending state, and the checksum
paragraph re-grounds on `[spec:]` and the board tags {design-bearing}.
**Not yet applied.** Re-phrase, never append:

- **The Design-pending bullet.** The entry sits in a design-pending section, which
  excludes it from selection. The set definition and the icebox clause stay. The
  sentences "Every entry in the set carries the tag…" and "The token names the
  *state*…" go. A promotion still drops the two deferred board tags, and
  `check-deferred-board-tags` assertion B is still the checksum on that drop.
- **The bidirectional-rule paragraph.** "goes to the deferred section
  `[design-pending]`" becomes "goes to the deferred section".
- **The checksum paragraph** ("The tag is a **checksum on the promotion move**…") is
  re-phrased to cover `[spec:]` alone. Its redundancy is the mechanism: a promotion
  crosses a section boundary *and* adds the ref, so a feature entry without the ref
  and a design-pending entry with one each red. The admission test becomes: **a
  state tag is admitted only when it carries information its section does not.** A
  tag restating the section that holds it is the two-sources defect and is refused,
  as `[icebox]` was and as `[design-pending]` now is. The attribute-tag paragraph
  after it stays as it is.
- **Merge step 4, the demotion branch.** "under its design-pending tag" becomes
  "into its design-pending section".
- **§Layout and configuration.** The generic-vocabulary sentence drops
  `[design-pending]` from its queue-tag list. `CANON_KIT_ACTIVE_SECTIONS` becomes
  "sections where a `[spec:]`-tagged entry is misfiled unless the section is also a
  feature section". `CANON_KIT_DEFERRED_SECTION` becomes "the deferred section, the
  first member of the design-pending section set". The marker-spelling sentence
  near the end of §Layout and configuration drops `[design-pending]`, and its
  `prose-enum-exempt` reason is re-worded to match.
- **§The shared spec adapters, section grammar.** "one section set so the tag rule
  and the walk can never disagree on it" becomes "one section set so the pairing
  arms and the walk can never disagree on it".

### (2) check-amendment-queue: the tag arms become one retired-token arm

`native/src/gates/amendment_queue.rs` and §check-amendment-queue replace the three
tag arms with a single retired-token arm. The `[spec:]` arms are untouched
{design-bearing}. **Not yet applied.**

- **Removed:** the tag-in-an-active-entry arm and the tag-in-active-prose arm (both
  under a). The every-design-pending-entry-carries-the-tag arm (b, first half) goes
  too.
- **Kept:** a feature entry without `[spec:]`, and a `[spec:]` entry misfiled in an
  active non-feature section (a). A design-pending-section entry carrying `[spec:]`
  (b, its remaining half). The on-disk pairing (c).
- **Added (d), retired token:** any line in a feature, active or design-pending
  section that contains the literal `[design-pending]` is red. This covers lead lines
  and body prose alike. The finding line is
  `[design-pending] is retired — section membership is the state; delete the tag`.
  Prose about the state spells it without brackets, which is the rule
  queue-kit/SPEC.md §check-tag-lead-line already gives for prose about a tag. The done
  section and every other heading class skip the arm, as they skip the others.
- **Why a red and not an inert leftover.** A retired token left inert keeps the two
  sources alive in every adopter queue with nothing to say so. The owner doc already
  rules this for queue grammar: queue-kit/SPEC.md §check-queue-entry-budget
  assertion (D) refuses the retired `ruled:` token "because the grammar's inducer was
  habit, and habit re-mints a spelling the record still shows". This tag has more
  habit behind it than `ruled:` had: hundreds of lead lines in this repo's own queue,
  plus every shipped template. The migration is mechanical, and the finding names it.
- **Help and clean lines.** The help line drops "tag every design-pending-section
  entry [design-pending]" and names the retired-token remedy instead. The clean line
  "every design-pending-section entry tagged" becomes "no retired design-pending
  tag".
- **The coverage-limit paragraph** is re-phrased. The done section is still an exempt
  population. What it is exempt from is now the `[spec:]` arms and arm (d). Its example
  becomes an entry carried into done with its `[spec:]` tag. Its sentence about the
  guard being total over the *promotion* moves now rests on `[spec:]`.
- **Fixture pair.** `bad/TASK-QUEUE.md` loses its three tag-arm cases and gains a
  lead-line `[design-pending]` and a body-prose `[design-pending]` in the deferred
  section, plus one in an active section. `bad/expect.txt` swaps its three tag-arm
  lines for the retired-token line. `good/TASK-QUEUE.md` drops the tag from its
  deferred entry, and that entry names the state in prose without brackets.
- **Merge coordination.** The sibling unit `amendment-owner-position-citation` also
  edits this module and this fixture pair. Whichever batch lands second rebases onto
  the first. Neither amendment's arm reads the other's input.

### (3) queue-kit: the tag leaves the algebra, the grammar and the gates

queue-kit's surfaces drop the tag and re-phrase every passage that used it as an
example {mechanical}. **Not yet applied.**

- **§The queue format.** The body field `Why [design-pending]` is renamed
  `Why design-pending`. The renamed field is a body field and not a tag, so the two
  meanings no longer share a spelling.
- **§The icebox tier.** The grammar block becomes
  `- **<slug>** — <one sentence: what it is, and why it is dormant>`. The bullet "It
  carries the same `[design-pending]` tag every deferred entry carries" is re-phrased
  as "**No state tag.** Section membership *is* the state, for this tier as for the
  deferred section; a tag restating its own section is the two-sources defect." The
  "why it is dormant" clause stays.
- **§The tag algebra.** The `[design-pending]` bullet is deleted.
- **§The roadmap arm.** The honest-limit sentence compares `[spec: <file>]` with
  `[design-pending]`. It is re-phrased to state the spec tag's width against the wrap
  floor without that comparison.
- **§check-deferred-board-tags assertion B.** "A promotion drops both, as it drops
  `[design-pending]`" becomes "A promotion drops both".
- **§check-tag-lead-line.** The governed set drops `[design-pending]`. The reflow list
  drops "masks a design-pending state".
- **§check-task-conservation.** The example shape `- **<slug>** [design-pending] — …`
  becomes `- **<slug>** [cost: …] — …`. The last sentence becomes "the tags are
  dropped by that reduction rather than swapped".
- **§check-queue-prose-precondition.** The third rewrite still strips the unbracketed
  word `design-pending`. The state name ends in a trigger word, and prose names the
  state and its section set. The rationale is re-phrased from "the queue's own tag
  name" to "the queue's own state name". "the bracket rewrite reaches
  `[design-pending]`" becomes "the bracket rewrite reaches any bracketed tag".
- **§Out of scope.** "around `[design-pending]`/`[spec:]`" becomes "around the
  design-pending sections and `[spec:]`".
- **`native/src/gates/tag_lead_line.rs`.** `"design-pending]"` leaves `CLASSES`. The
  unit test's design-pending assertions are re-pointed at `attend]`, the other bare
  class. `native/src/emit/enum_sets.rs`'s test swaps its `design-pending` assertion
  for `spec`.
- **`native/src/gates/queue_prose_precondition.rs`.** `TAG_WORD_RE_SRC` keeps its
  value. The error message and any comment naming it a tag name are re-worded.
- **Tests and fixtures carrying the tag as an arbitrary sample:** its test strings
  in `native/src/gates/deferred_board_tags.rs`, `native/src/gates/task_conservation.rs`,
  `native/src/emit/queue_index.rs` and `native/src/emit/queue_counts.rs`. For
  `queue_counts.rs`, the `render_by(…, "design-pending")` case is re-keyed to
  `roadmap` and its expected string follows. `queue-kit/gate-tests/queue-counts.test.sh`
  gets the same re-key. The tag is deleted from the fixture queues of
  `check-deferred-board-tags`, `check-queue-entry-budget`, `check-roadmap-fresh`,
  `check-queue-prose-precondition` and `check-tag-lead-line`. The tag-lead-line
  `bad/` pair's stranded-tag case moves to another governed class, and its
  `expect.txt` follows.
- **`queue-kit/templates/TASK-QUEUE.md`** drops the tag from `example-deferred`.
  **`queue-kit/README.md`** drops it from the tag roster. So does
  **`queue-kit/checks/check-tag-lead-line.gate`**'s `# spec:` line.

### (4) lifecycle-kit: templates and SPEC name the section, not the tag

Every lifecycle-kit passage that says to file or restore "a `[design-pending]` entry"
says "a deferred entry" instead {mechanical}. **Not yet applied.**

- `lifecycle-kit/templates/stages/close.md`: "files instead as a Deferred
  `[design-pending]` entry" becomes "files instead as a Deferred entry". The →promote
  disposition becomes "file a deferred queue entry carrying its `[cost:]` and
  `[surface:]` tags".
- `lifecycle-kit/templates/stages/scope.md`: the exit-condition example "no
  design-pending tag left in the active queue" becomes "every active feature entry
  spec-ready".
- `lifecycle-kit/SPEC.md` §Deviation transitions, the demote ritual: "restoring its
  design-pending tag" becomes "restoring its board tags". §The committed gap inbox:
  "promoted to a deferred `[design-pending]` entry" becomes "promoted to a deferred
  entry". §check-stage-entry assertion B: "files as Deferred `[design-pending]`"
  becomes "files as a Deferred entry".
- `native/src/emit/enter_stage.rs`: the help text "a deferred [design-pending] entry"
  becomes "a deferred entry". `native/src/emit/file_gap.rs`'s test fixture and
  `lifecycle-kit/gate-tests/file-gap-recurrence.test.sh` drop the tag from their
  sample entry.

### (5) The consumer sweep: this repo's queue and the remaining surfaces

The queue and every remaining carrier drop the tag in the **same commit** as delta 2's
arm (d). Split across commits, the queue reds under the new arm or the old one
{mechanical}. **Not yet applied.**

- **`TASK-QUEUE.md`.** Delete ` [design-pending]` from every lead line in the Deferred
  and Icebox sections. Rename the body field `Why [design-pending]` (with or without
  backticks, including its `Why [design-pending], <qualifier>` variants) to
  `Why design-pending`. Re-phrase every remaining bracketed mention in a body to the
  unbracketed state name. The queue preamble and the Icebox preamble get the same
  treatment. The oracle is delta 2's arm (d) over the file, clean.
- **`drift-kit/smoke/install.sh`**: its sample queue entries drop the tag.
- **`gate-sdk/SPEC.md`** §upgrade-smoke: in the port-cut refusal passage, "settling a live
  `[design-pending]` fork" becomes "settling a live design-pending fork".
- **`README.md`** (the kit-map row) and **`docs/queue-kit/index.md`**: the tag roster
  drops `design-pending`.
- **`.workflow/release-declarations.md`**: one bullet. A vendoring consumer's queue
  reds under `check-amendment-queue` until the tag is deleted. The migration is to
  delete ` [design-pending]` from every lead line and to un-bracket body mentions.
- **`docs/posts/2026-07-31-checkwright-v0-18-0.md`** is a dated release post that
  records the tag as it shipped. It is left as written.

## Producers and consumers

- **Arm (d), the retired-token finding (delta 2).** Producer: `check-amendment-queue`
  over the queue file, on the generated pre-commit hook, `run-gates.sh` and CI. Its
  enabling config is the existing section knobs, set in every deployed configuration
  (zero-config install). Consumers: the committing session through the output
  contract, and the `--run-gate-tests` arm through the fixture pair. The finding has
  one field, the offending line, and the committing session reads it to find the
  line to edit.
- **The re-grounded checksum (delta 1).** No new producer. It is the existing
  `[spec:]` arms of `check-amendment-queue` and assertions A and B of
  `check-deferred-board-tags`, which is `install: on-surface` and registered in this
  repo's `scripts/gates.list`. **Honest limit, stated on the merged canon-kit
  paragraph:** a consumer that does not register `check-deferred-board-tags` loses the
  debt-entry move checksum the tag gave. An icebox one-liner moved into an active
  section by accident is uncaught everywhere, because the icebox carries no board
  tags. Nothing records such a move ever having happened.
- **Roster-holding readers of the removed name (point 2, inverted).** The tag class
  table in `tag_lead_line.rs` is the derivation surface for `--emit-enum-sets`, and
  that feeds `check-prose-enum`. **Red condition:** check-prose-enum reds only on an
  *omitted* member (canon-kit/SPEC.md §check-prose-enum). So a prose roster that still
  lists `design-pending` after the set shrinks stays clean, and the roster edits in
  deltas 3 and 5 are for truth, not driven by a red. `check-tag-lead-line` stops
  governing the tag. A stranded leftover is still caught, now by arm (d).
- **Point 5 (narrowing).** Delta 2 narrows `check-amendment-queue` by three arms.
  Their readers are the committing session (no floor, no count, no find-none red) and
  the fixture pair's `bad/expect.txt`, whose three lines delta 2 replaces. Delta 3
  narrows `check-tag-lead-line`'s governed set. Its `bad/` fixture expects a stranded
  design-pending finding, and delta 3 moves that case to another class so the pair
  stays red on an equal count. `check-prose-enum`'s generated set shrinks by one
  member, which is monotone clean as stated above.
- **Point 6 (members).** The obliged corpus is every line carrying the literal in a
  task section of `TASK-QUEUE.md`, enumerated by `git grep -c -F "[design-pending]"
  TASK-QUEUE.md` (416 on 2026-09-17, 357 of them on top-level lead lines). Each
  line's satisfying value is the line with the token deleted (lead lines) or
  un-bracketed (bodies).

## Existing sections updated

- `canon-kit/SPEC.md` §The amendment lifecycle, §Merging an amendment step 4,
  §Layout and configuration (the generic-vocabulary sentence, two knob bullets, the
  marker-spelling sentence), §The shared spec adapters (section grammar)
  (delta 1).
- `canon-kit/SPEC.md` §check-amendment-queue: the invariant's arms, the coverage-limit
  paragraph (delta 2).
- `native/src/gates/amendment_queue.rs`: arm removal, arm (d), help and clean lines
  (delta 2).
- `canon-kit/gate-tests/check-amendment-queue/`: the `bad/` queue and `expect.txt`,
  and the `good/` queue (delta 2).
- `canon-kit/gate-tests/check-deprecation-task/good/TASK-QUEUE.md`: drop the tag from
  its sample entry (delta 3).
- `canon-kit/gate-tests/check-todo-task-liveness/good/TASK-QUEUE.md`: the same
  (delta 3).
- `queue-kit/SPEC.md` §The queue format, §The icebox tier, §The tag algebra,
  §The roadmap arm, §check-deferred-board-tags, §check-tag-lead-line,
  §check-task-conservation, §check-queue-prose-precondition, §Out of scope (delta 3).
- `queue-kit/README.md`: the tag roster (delta 3).
- `queue-kit/templates/TASK-QUEUE.md`: `example-deferred` (delta 3).
- `queue-kit/checks/check-tag-lead-line.gate`: the `# spec:` line (delta 3).
- `native/src/gates/tag_lead_line.rs`: `CLASSES` and its unit test (delta 3).
- `native/src/emit/enum_sets.rs`: its tag-vocabulary test (delta 3).
- `native/src/gates/queue_prose_precondition.rs`: the tag-word message and comments
  (delta 3).
- `native/src/gates/deferred_board_tags.rs`: test strings (delta 3).
- `native/src/gates/task_conservation.rs`: test string (delta 3).
- `native/src/emit/queue_index.rs`: test fixtures (delta 3).
- `native/src/emit/queue_counts.rs`: test fixture and the `render_by` re-key (delta 3).
- `queue-kit/gate-tests/queue-counts.test.sh`: the same re-key (delta 3).
- `queue-kit/gate-tests/`: the fixture queues of `check-deferred-board-tags`,
  `check-queue-entry-budget`, `check-roadmap-fresh`, `check-queue-prose-precondition`
  and `check-tag-lead-line`, and the last one's `bad/expect.txt` (delta 3).
- `lifecycle-kit/templates/stages/close.md`: the capture and →promote sentences
  (delta 4).
- `lifecycle-kit/templates/stages/scope.md`: the exit-condition example (delta 4).
- `lifecycle-kit/SPEC.md` §Deviation transitions, §The committed gap inbox,
  §check-stage-entry (delta 4).
- `native/src/emit/enter_stage.rs`: the help text (delta 4).
- `native/src/emit/file_gap.rs`: the test fixture (delta 4).
- `lifecycle-kit/gate-tests/file-gap-recurrence.test.sh`: the sample entry (delta 4).
- `TASK-QUEUE.md`: the sweep (delta 5). Also the Done move for
  `design-pending-tag-restates-its-own-section`, made by the build session that
  merges this file, before the drain stage is entered (all deltas).
- `drift-kit/smoke/install.sh`: sample entries (delta 5).
- `gate-sdk/SPEC.md` §upgrade-smoke: the port-cut refusal passage (delta 5).
- `README.md`: the queue-kit row's tag roster (delta 5).
- `docs/queue-kit/index.md`: the tag roster (delta 5).
- `.workflow/release-declarations.md`: the migration bullet (delta 5).
- `docs/posts/2026-07-31-checkwright-v0-18-0.md`: no edit, a dated post (delta 5).
- `docs/canon-kit/SPEC.md`: generated mirror, regenerated (all deltas).
- `docs/queue-kit/SPEC.md`: generated mirror, regenerated (all deltas).
- `docs/queue-kit/README.md`: generated mirror, regenerated (all deltas).
- `docs/lifecycle-kit/SPEC.md`: generated mirror, regenerated (all deltas).
- `docs/gate-sdk/SPEC.md`: generated mirror, regenerated (all deltas).
- <!-- update-target-exempt: generated projections, each rostered with its freshness gate and regen command in docs/site-architecture.md §Generated projections and their freshness gates --> `docs/enforcement.md` and the generated pre-commit hook.

Roster produced by `git grep -l -F "[design-pending]"` and
`git grep -n -F "design-pending"` over the tracked tree on 2026-09-17, with each
hit's enclosing heading read by `awk`. It is a floor that build re-derives.

## Retired spellings

- `[design-pending]` — the queue tag, on lead lines and as a bracketed body mention
  (deltas 1, 2, 3, 4 and 5).

## Definition of Done

- [ ] **Causal completeness** — every point of canon-kit's causal-completeness check
      holds for arm (d) and for the removal of the three arms and the class.
- [ ] **Instruction surfaces: instruction only** — the close and scope template edits
      carry no grounds; the grounds live in canon-kit §The amendment lifecycle.
- [ ] **Merged with no information lost** — the two-sources ground, the re-grounded
      checksum, the retired-token-red ground and the move-checksum honest limit
      survive in the merged canon-kit and queue-kit prose.
- [ ] **Attested reproduction** — a scratch queue carrying `[design-pending]` on a
      deferred lead line reds `check-amendment-queue` with the retired-token finding.
      The same queue with the token deleted is clean.
- [ ] **One-commit sweep** — delta 2's arm and delta 5's queue sweep land together,
      and the full battery is green at that commit.
- [ ] **Queue move placed before the drain stage** — the Done move lands in the
      session that merges this file, before the drain stage is entered.
- [ ] **Amendment deleted** — this file removed on merge; none remain at the repo
      root (`ls SPEC-*.md`) once the iteration's last batch lands.
- [ ] **Removals propagated** — `check-amendment-retired-spelling` is clean for the
      declared spelling over the whole tracked tree.
- [ ] **Gaps filed** — cross-component gaps found during the work go to the gap inbox.
