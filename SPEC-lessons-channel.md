# SPEC amendment: lessons-channel

**The Lessons Learned channel becomes consumer-optional, on by default, and this repo runs without it** (operator direction, 2026-10-01, lead session). queue-kit and lifecycle-kit ship the channel to every adopter: a fixed-spelling `## Lessons Learned` section, the `[attend]` attention tag and the configured harvest tags, close's step 1 with `check-lesson-disposition` and its evidence file, the lesson-sink arm, and validate's filing rule routing method observations there. This repo's section has held no bullet since its last harvest, and every close still walks the step over it. The gap inbox, the close drain's fix-forward-promote-discard set and the sessions' own records now carry each function the channel served, with one loss named below. Policy-as-choice gives the result its shape: the kit keeps the channel and ships it on, a consumer selects it off, and this repo binds off.

The change spans queue-kit's queue format, lifecycle-kit's close and validate templates, and this repo's bindings, so this amendment sits at the repository root.

**Read at authoring:**

- **Use.** `git log -p -- .workflow/lesson-evidence.txt`: the last disposition stamp is from 2026-09-16, a →rule. Across the file's history the dispositions are mostly →rule, then →task and →discard, with →harvest three times between 2026-07-10 and 2026-07-16. The last `[attend]` lead line was written 2026-07-11, and the queue's section holds no bullet today.
- **The essay harvest's reader is confirmed live.** The `[essay]` tag's body stages to `.workflow/essay-harvest.md` (`.claude/commands/close.md`, `harvest-routing`), and a harvest pass outside this tree reads that file. The same pass also sweeps this repo's session record directly, so the staged file is one of its two inputs. The operator chose to drop that inbox with the channel (operator direction, 2026-10-01, lead session): its last staged body is from 2026-07-16, and the pass's own sweep of the session record covers the same sessions.
- **Every reader degrades to "no lesson" on an absent section**, so off needs no new knob, read off each reader's code:
  - `--enter-stage`'s boundary check reads the section's bullets (`native/src/emit/enter_stage.rs`), and an absent section has none, so the boundary passes;
  - `check-lesson-disposition` reads the section's entries at HEAD and in the worktree (`native/src/gates/lesson_disposition.rs`, `lessons_of`), so with neither carrying one nothing is removed and it is clean, provided its evidence file exists;
  - the queue-index arm's attention block and `check-tag-lead-line` scan the section's lines and print or find nothing (`native/src/emit/queue_index.rs`, `native/src/gates/tag_lead_line.rs`);
  - `check-queue-sections` reds a missing required heading, and `Lessons Learned` is in `QUEUE_KIT_REQUIRED_SECTIONS`' default (`native/src/knobs/queue_kit.rs`), so a consumer turning the channel off overrides that array, which it may already do.
- **The evidence file stays.** It is a kit built-in of the boundary truncate and the `merge=iteration-scoped` set (lifecycle-kit/SPEC.md §bin/enter-stage.sh, §check-merge-attrs), and `check-lesson-disposition` is fail-closed on its absence. A consumer running without the channel keeps the header-only file the installer seeds, and the gate stays clean over it. Whether this repo keeps the gate registered is `lifecycle-queue-value-audit`'s verdict, which follows this decision. This amendment leaves the registry alone.
- **The Lessons close-surface row** (queue-kit/SPEC.md §The tag algebra) reads `-` for its state whenever the queue file exists (lifecycle-kit/SPEC.md §The close-surfaces emit arm), so a consumer without the section meets a row and no `absent` finding.

## What changes

### (1) queue-kit states the channel optional

In queue-kit/SPEC.md, two passages are re-phrased. {design-bearing} {user-facing: operator direction 2026-10-01, lead session, the channel consumer-optional and shipped on} **Not yet applied.**

§The tag algebra, after the paragraph opening "Two tags ride **Lessons Learned** entries", its close-surface declaration and its two tag bullets, gains a closing paragraph:

> **The section is consumer-optional and ships on.** A consumer that leaves it out of its queue and out of `QUEUE_KIT_REQUIRED_SECTIONS` runs without the lesson channel, and every reader of the section reads no lesson, which is that consumer's off. The default required set names it, so an adopter keeps the channel until it chooses otherwise.

§Layout and configuration, the `QUEUE_KIT_REQUIRED_SECTIONS` bullet, gains a closing sentence: "Leaving `Lessons Learned` out is how a consumer turns the lesson channel off (§The tag algebra)."

In the same commit, the comment directive in `native/src/emit/enum_sets.rs` citing "queue-kit/SPEC.md §The Lessons Learned channel", a heading the SPEC does not carry, cites §The tag algebra instead.

### (2) The close and validate templates run the channel only where it exists

Four template sentences gain the condition. {mechanical} {user-facing: operator direction 2026-10-01, lead session, the channel consumer-optional and shipped on} **Not yet applied.**

- `lifecycle-kit/templates/stages/close.md`, the opening: "Exit condition: Done and Lessons Learned sections cleared (harvestable lessons promoted first)." becomes "Exit condition: the Done section cleared, and the Lessons Learned section too where the queue keeps one (harvestable lessons promoted first)."
- `close.md` step 1's lead, "**Process Lessons Learned** → durable rules or debt tasks, then clear the section.", becomes "**Process Lessons Learned** → durable rules or debt tasks, then clear the section. A queue keeping no such section skips this step."
- `lifecycle-kit/templates/stages/validate.md`'s filing rule, "an observation about how the work should be done ⇒ the lessons section, dispositioned at close.", becomes "an observation about how the work should be done ⇒ the lessons section where the queue keeps one, else the gap inbox (`--emit file-gap`), dispositioned at close either way."
- `validate.md`'s red triage, "first grep the queue's deferred/lessons sections", becomes "first grep the queue's deferred section, and its lessons section where it keeps one".

### (3) lifecycle-kit carries the grounds and the replacement map

lifecycle-kit/SPEC.md gains one paragraph and re-phrases two sentences. {design-bearing} **Not yet applied.**

§templates/stages/, *The close template*, gains as its first paragraph:

> **The lesson channel is consumer-optional, because the gap inbox carries what it carries.** A lesson's four dispositions each have a route without it: a →rule is the drain's →fix when it adds no governed name and its →promote otherwise, a →task is →promote, and a →discard is →discard (§The committed gap inbox). A →harvest needs no section either, since the lesson-sink arm takes its body on stdin (queue-kit/SPEC.md §The lesson-sink arm), and a consumer may harvest its session record instead. The loss is `[attend]`, a lesson's injection into every later session of the same iteration, which only a live lead's dispatch prompts carry without it. So the kit ships the channel on and a consumer turns it off by leaving the section out (queue-kit/SPEC.md §The tag algebra), and close's step 1 and the boundary's Lessons check each read an absent section as an empty one.

§bin/enter-stage.sh, the sentence "**The boundary entry refuses on a non-empty `## Lessons Learned`**" gains after its first sentence: "A queue keeping no such section passes the check."

§check-lesson-disposition gains a closing sentence: "A queue keeping no Lessons section at HEAD has nothing to disposition and the gate is clean, over the header-only evidence file the kit's built-ins keep."

### (4) This repo binds the channel off

This repo's queue, queue config, close binding and ignore file take the off state, in one commit. {mechanical} {user-facing: operator direction 2026-10-01, lead session, the channel off here with the `[essay]` tag, its sink row and the harvest-routing step dropped} **Not yet applied.**

- `TASK-QUEUE.md` drops its `## Lessons Learned` heading.
- `scripts/queue-config.knobs` binds `QUEUE_KIT_REQUIRED_SECTIONS` to `Iteration:`, `New Features`, `Technical Debt`, `Deferred` and `Done`, one `QUEUE_KIT_REQUIRED_SECTIONS[] =` line each, under the policy-calibrations comment. It drops `QUEUE_KIT_LESSON_TAGS[] = essay`, and the header comment drops its lesson-harvest-tag clause.
- `.claude/commands/close.md`'s `harvest-routing` binding becomes "none — this repo keeps no Lessons Learned section (`scripts/queue-config.knobs`), so step 1 skips and no harvest routes." Its `[essay]` sub-bullet and the `.workflow/essay-harvest.md` close-surface row under it go, and the `housekeeping` binding's clause "the essay-harvest row to the essay merge named under `harvest-routing`" goes with them.
- `.gitignore` drops the `.workflow/essay-harvest.md` line and the two comment lines above it, since nothing writes the file once no harvest tag is configured.

### (5) The site mirrors follow

`docs/queue-kit/SPEC.md` and `docs/lifecycle-kit/SPEC.md` are regenerated with `bash gate-sdk/bin/run-gates.sh --emit docs-mirror --write` in the commits landing deltas 1 and 3. {mechanical} `docs/footprint.md` and `docs/value.md`'s rollup block are regenerated (`--emit footprint > docs/footprint.md`, `--emit value-rollup --write`) in delta 2's commit, whose template markdown the footprint measures. That commit appends the unit's Behavior-changes bullet to `.workflow/release-declarations.md`: the lesson channel is consumer-optional, nothing to do for a consumer keeping it (lifecycle-kit/templates/stages/build.md, *Declare what a vendoring consumer will meet*).

## Producers and consumers

- **The off state (deltas 1 and 4).** Producer: a consumer's queue without the section, with `QUEUE_KIT_REQUIRED_SECTIONS` set to match. This repo's delta 4 sets it, so the state has a deployed producer. Consumers, each named with its red condition:
  - `check-queue-sections` reds a required heading missing and a heading appearing twice. With the override it no longer requires the section.
  - `check-lesson-disposition` reds a lesson removed without a stamp, and exits 2 on a missing evidence file. Neither occurs: no lesson exists to remove and the file stays.
  - `--enter-stage`'s boundary check refuses on a non-empty section. An absent one is empty.
  - The queue-index arm and `check-tag-lead-line` find nothing to print or check.
  - `check-prose-enum` reads the `queue-lessons-tag` set, `[attend]` plus `QUEUE_KIT_LESSON_TAGS` (`native/src/emit/enum_sets.rs`). Dropping `essay` shrinks the set, so a prose enumeration of the lesson tags that names `essay` reds until it drops the name. Build runs the gate in delta 4's commit.
- **The validate filing route (delta 2).** Producer: a validate session holding a method observation in a queue without the section. Consumer: the gap inbox's drain at close, by its existing set. No new interface.
- **No new knob, tag, name or interface.** The off switch is the existing required-sections array and the section's absence.
- **Narrowing.** Delta 4 removes a section from the queue file, which every queue reader scans. The readers above with a red condition are named, and no reader asserts a count or minimum over lesson bullets.

## Existing sections updated

Roster produced by `git grep -n -i "lesson" -- ':!TASK-QUEUE.md' ':!docs/posts' ':!*/gate-tests/*'`, with each hit read.

- `queue-kit/SPEC.md` — §The tag algebra's Lessons passage and §Layout and configuration's `QUEUE_KIT_REQUIRED_SECTIONS` bullet (delta 1).
- `native/src/emit/enum_sets.rs` — the comment directive's section citation (delta 1).
- `lifecycle-kit/templates/stages/close.md` — the opening and step 1 (delta 2).
- `lifecycle-kit/templates/stages/validate.md` — the red triage and the filing rule (delta 2).
- `lifecycle-kit/SPEC.md` — §templates/stages/ *The close template*, §bin/enter-stage.sh and §check-lesson-disposition (delta 3).
- `TASK-QUEUE.md`, `scripts/queue-config.knobs`, `.claude/commands/close.md`, `.gitignore` — this repo's off binding (delta 4).
- `docs/queue-kit/SPEC.md`, `docs/lifecycle-kit/SPEC.md` — the regenerated mirrors (deltas 1, 3 and 5).
- `docs/footprint.md`, `docs/value.md` — the regenerated footprint and rollup block (delta 5).
- `.workflow/release-declarations.md` — the unit's Behavior-changes bullet (delta 5).

The other hits keep their text, since each describes the channel where it exists: the lesson-sink arm, the queue-index attention block, `check-tag-lead-line`, the kit READMEs' install skeleton, the close-surface rows and `check-lesson-disposition`'s grammar.

## Retired spellings

- None — every delta conditions or adds text, and the one spelling this repo drops, the `essay` tag, stays a valid consumer value in the kit's grammar.

## Definition of Done

- [ ] **Causal completeness** — every point of canon-kit/SPEC.md §The causal-completeness check holds for each new state, event, interface and obligation.
- [ ] **Instruction surfaces: instruction only** — replacement text for a template, agent definition or shim carries no grounds; a delta places them (the Content-tiering / SSOT rule).
- [ ] **Merged with no information lost** — each addition re-phrases the canonical-spec text it refines rather than appending to it; the merged spec reads as one document a reader who never saw the amendment can use alone.
- [ ] **Amendment deleted** — this file removed on merge; no root-level amendment of this unit remains (`ls SPEC-*.md`).
- [ ] **Removals propagated** — every name this change retired is declared in `## Retired spellings` above, and `check-amendment-retired-spelling` runs each declaration against the whole tracked tree, not against the specs alone.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed as debt tasks (a build-time causal gap is resolved that session, not deferred).
- [ ] **The entry moves** — `lessons-learned-channel-audit` moves to Done with `--queue done` in the commit deleting this file, at its build batch, before the drain stage and before `lifecycle-queue-value-audit` rules on `check-lesson-disposition`.
