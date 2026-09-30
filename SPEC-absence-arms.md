# SPEC amendment: absence-arms

**The Absence statements rule has two shapes that need no judgment, and neither gates today.** doctrine-kit/DOCTRINE.md's rule leaves hand prose to judgment, with an audit-roster class as its cadence, because whether a reader must know a question was considered is judgment in general. Two shapes decide without it:

- **A section whose whole body is a placeholder or one negative-existential sentence**, on a surface where the heading alone already shows the emptiness. Whether that surface is such a place is the consumer's call, made once by naming the surface. The sentence needs no reading once the surface is named.
- **A placeholder standing where a queue entry would**, in a work queue whose format says an empty section is its heading alone.

This amendment spans canon-kit, queue-kit and doctrine-kit, so it sits at the repository root.

**Measured at authoring, and correcting the entry's premise.**

- `check-task-names` already reds a column-0 `- none` in a task section, as a bullet outside every entry. What passes it and `check-queue-hygiene` alike is an indented `None` or `- none` outside every entry, and a `- none` under Done, read as the bare slug `none`. The runs were `bash gate-sdk/bin/run-gates.sh --only <gate> -- <scratch queue>`, one gate per call.
- A sweep of the tracked kit SPECs, READMEs, `docs/*.md`, CLAUDE.md, TRAJECTORY.md and ROADMAP.md finds no section whose whole body is one negative-existential sentence. The same sweep over TASK-QUEUE.md's Icebox finds one-sentence entry bodies that open with "No". Each describes a gap, and none states its own section's emptiness. So the queue stays out of arm (a)'s corpus, and arm (b) covers the queue.
- The live queue carries no placeholder line (`grep -n -i -E '^\s*(- )?(none|n/a|nil|tbd|nothing|empty|-|—)\.?\s*$' TASK-QUEUE.md` prints nothing).
- This repository's `CANON_KIT_PROSE_TELL_GLOBS` is `docs/*.md`, which reaches no `SPEC-*.md` amendment.

## What changes

### (1) check-prose-tells gains assertion G, the absence section

canon-kit's `native/src/gates/prose_tells.rs` {design-bearing}: a new corpus, `CANON_KIT_PROSE_TELL_ABSENCE_GLOBS`, read and walked apart from `CANON_KIT_PROSE_TELL_GLOBS`, and one assertion over it.

G reds a section whose whole body is a placeholder or one negative-existential sentence. A **section** here is a heading of any level and the lines up to the next heading of any level, a narrower span than C's and F's `##` sections, so G carries its own section tracker.

- **The body** is the section's non-blank lines, less HTML-comment-only lines, less any generated `<!-- name:begin -->`…`<!-- name:end -->` region, which is the emitter's. A section whose body is empty, or holds only a generated region, is clean: a heading alone is the shape the rule asks for.
- **A placeholder** is the body's one line reduced to a lone token: its bullet marker, emphasis and a trailing period stripped, and what remains `None`, `N/A`, `Nothing`, a hyphen or an em dash, compared case-insensitively.
- **A negative-existential sentence** is a body that is one unit holding one sentence (the shared split, §The shared spec adapters) that opens with `None`, `Nothing`, `No` and a word, `There is no`, `There are no`, `Not applicable` or `N/A`.

The detection grammar is implementation, owned by the gate source as the other assertions' are, and the fixture pair states its boundary. A two-sentence body is clean, since its second sentence says why and is read as an explanation. The `prose-tell-exempt:` valve reaches G on the heading's line or the body's.

The empty default is the off position, and no default reaches all markdown. The finding names the file, the heading's line and the section title, and G's `help:` line reads *delete the body so the heading stands alone, or, where a reader must tell considered-and-none from unfilled, keep the token and take the valve*.

The fixtures change on both sides:

- `bad/` gains a section holding "There are no known limitations." and a section holding a lone `- None`, with the absence glob set in its `scripts/canon-config.knobs`.
- `good/` gains a heading-only section, a two-sentence absence with its reason, and a generated region holding an absence sentence.

### (2) §check-prose-tells and §Layout and configuration state G

canon-kit/SPEC.md {mechanical}. **Not yet applied.**

In §check-prose-tells, the sentence "It runs six mechanical assertions over each surface …" gains "and a seventh, the absence section (G), over its own surface set". After F's bullet comes:

> - **G. Absence section** — over `CANON_KIT_PROSE_TELL_ABSENCE_GLOBS` alone: a section, a heading of any level to the next, whose whole body is a placeholder (`None`, `N/A`, a dash) or one sentence opening a negative existential (`None`, `No …`, `Nothing`, `There is no`), generated regions and comment lines held out. The surface is the consumer's: bind the hand prose whose sections show their emptiness by the heading alone, and leave out a checklist or declaration grammar whose `None` token a reader needs (an amendment's `## Retired spellings`, a work queue). doctrine-kit's Absence statements rule is the rule G mechanizes the decidable shape of.

The criterion-4 paragraph's "its corpus is a pure glob expansion of the configured prose-surface set" becomes "its corpus is a pure glob expansion of the two configured prose-surface sets".

In §Layout and configuration, after the `CANON_KIT_PROSE_TELL_GLOBS` bullet:

> - `CANON_KIT_PROSE_TELL_ABSENCE_GLOBS` — array of repo-root-relative globs, default empty ⇒ assertion G off: the hand-prose surfaces `check-prose-tells` holds to the absence section (§check-prose-tells).

In the corpus-knob roster paragraph ("**A corpus knob below widens the declaring gates' *triggers* …**"), `CANON_KIT_PROSE_TELL_ABSENCE_GLOBS` joins the list after `CANON_KIT_PROSE_TELL_GLOBS`.

`native/src/knobs/canon_kit.rs` gains the indexed row with an empty default. The member's knob list in `native/src/gates/mod.rs` gains the name. `canon-kit/checks/check-prose-tells.gate`'s `couples=` gains `knob:CANON_KIT_PROSE_TELL_ABSENCE_GLOBS`.

### (3) check-task-names reds a placeholder standing for an entry

queue-kit's `native/src/gates/task_names.rs` {design-bearing}: a new knob, `QUEUE_KIT_PLACEHOLDER_TOKENS`, indexed and case-insensitive. Its default is the shipped set `none`, `n/a`, `n-a`, `nil`, `tbd`, `nothing`, `empty`, `-` and `—`, and empty turns the arm off. The arm reds three shapes:

- a line in a task section outside every entry whose content is a member, after its indentation, a bullet marker, emphasis and a trailing period are stripped. A column-0 bullet the existing outside-every-entry finding already reports is not reported twice.
- a done-section bare slug that is a member.
- an entry heading whose slug is a member.

The finding names the line and the token. Its `help:` line reads *an empty section is its heading alone: delete the line*.

The fixtures change on both sides:

- `bad/TASK-QUEUE.md` gains an indented `None` under an active section and a `- none` under Done.
- `good/` gains an entry whose slug only starts with a member (`none-left-over`) and an indented section preamble sentence.

### (4) §check-task-names and §Layout and configuration state the arm

queue-kit/SPEC.md {mechanical}. **Not yet applied.** The §check-task-names invariant gains, before "every `[blocked-by: X]`":

> no line standing where an entry would, outside every entry of a task section, as a done slug or as an entry heading, is a placeholder token from `QUEUE_KIT_PLACEHOLDER_TOKENS`, since an empty section is its heading alone (doctrine-kit's Absence statements rule);

In §Layout and configuration, after `QUEUE_KIT_PROSE_LEADS`:

> - `QUEUE_KIT_PLACEHOLDER_TOKENS` — array of placeholder tokens `check-task-names` reds where an entry would stand, matched case-insensitively, default the shipped set (`none`, `n/a`, `nil`, `tbd`, a dash, …; `--emit knob-roster` prints it whole); empty turns the arm off.

`native/src/knobs/queue_kit.rs` gains the row, and the member's knob list in `native/src/gates/mod.rs` gains the name.

### (5) The doctrine names the two gated shapes

doctrine-kit/DOCTRINE.md, Absence statements, the *Enforced by* clause's last sentence {mechanical}. **Not yet applied.**

> On hand prose the branch is judgment (the Enforcement-first false-positive carve-out), with a consumer audit-roster class as its cadence (…), except two shapes that need none and gate where a consumer binds them: a section whose whole body is a placeholder or one negative-existential sentence ([canon-kit/SPEC.md](../canon-kit/SPEC.md) §check-prose-tells), and a placeholder standing where a queue entry would ([queue-kit/SPEC.md](../queue-kit/SPEC.md) §check-task-names).

The digest is unchanged, so no agent file's digest bullet moves.

### (6) This repository binds arm (a) and narrows its roster class

`scripts/canon-config.knobs` {mechanical}: `CANON_KIT_PROSE_TELL_ABSENCE_GLOBS` binds `*/SPEC.md`, `*/README.md`, `README.md`, `doctrine-kit/DOCTRINE.md`, `docs/*.md`, `CLAUDE.md`, `TRAJECTORY.md` and `ROADMAP.md`, under a `comment-tier-exempt:` comment. The comment says the queue stays out because its one-sentence entry bodies describe gaps and arm (b) holds it, and that amendments stay out because their `## Retired spellings` token is a declaration a reader needs.

`.workflow/audit-roster.txt`, class `absence-statement`, scope: "un-gateable, since whether a reader must know a question was considered is judgment" becomes "the two decidable shapes gate (check-prose-tells assertion G over the bound surfaces, check-task-names' placeholder arm); the rest is un-gateable, since whether a reader must know a question was considered is judgment".

**Inferred, cannot run before build:** the binding reds nothing on the live tree, as the authoring sweep's emulation of G's predicate found — assertion G does not exist until delta 1 lands, and the gate is the oracle.

### (7) The release declaration

`.workflow/release-declarations.md` {mechanical}, under Tightened gates:

> - `check-prose-tells` — gains assertion G, armed only once you set the new `CANON_KIT_PROSE_TELL_ABSENCE_GLOBS` (default empty, off): a section on those surfaces whose whole body is a placeholder or one sentence like "There are no known issues." reds. Nothing to do while it stays empty; to arm it, bind hand prose only, never a checklist or a work queue.
> - `check-task-names` — reds a placeholder token (`None`, `n/a`, a dash, from the new `QUEUE_KIT_PLACEHOLDER_TOKENS`, shipped set on by default) standing outside every entry of a task section, as a done slug or as an entry heading. Delete the line: an empty section is its heading alone. Set the knob empty to switch the arm off.

## Producers and consumers

- **Assertion G.**
  - Producer: `check-prose-tells`' per-file walk, over the new corpus, on the generated pre-commit hook, `run-gates.sh` and CI.
  - Enabling config: `CANON_KIT_PROSE_TELL_ABSENCE_GLOBS`, which this repository sets in delta 6.
  - Consumer: the committing session through the output contract. The finding's file, line and title are read at the one scan, and nothing persists.
- **The placeholder arm.**
  - Producer: `check-task-names` over the queue file.
  - Enabling config: the shipped default, so every consumer registering the gate runs it.
  - Consumer: the committing session, and the `--queue` verbs, whose post-write battery run reads the verdict.
- **Roster-holding readers of the minted names.**
  - Each kit's knob table validator refuses an undeclared name, so both rows land with their readers.
  - `check-kit-ref-liveness` and `check-docs-cmd` assertion B resolve each name exactly, through the static table the rows land in. `CANON_KIT_PROSE_TELL_` is no declared family, as that gate's own unit test holds.
  - `check-knob-citation` and `check-knob-default-coupling` read the new §Layout and configuration bullets. The queue-kit bullet names its shipped set as `QUEUE_KIT_PRECONDITION_REGEX`'s bullet and canon-kit's `CANON_KIT_PROSE_TELL_PHRASES` bullet already do, a few members and the knob roster for the whole.
- **The upgrade.** The placeholder arm is on by default, so an adopter's queue carrying `- none` under Done reds on upgrade. The Tightened-gates bullet in delta 7 is that allowed red.

## Existing sections updated

- `native/src/gates/prose_tells.rs`, `canon-kit/gate-tests/check-prose-tells/good/` and `bad/`: assertion G (delta 1).
- `canon-kit/SPEC.md` §check-prose-tells and §Layout and configuration, `native/src/knobs/canon_kit.rs`, `native/src/gates/mod.rs` and `canon-kit/checks/check-prose-tells.gate` (delta 2).
- `native/src/gates/task_names.rs` and `queue-kit/gate-tests/check-task-names/good/` and `bad/`: the placeholder arm (delta 3).
- `queue-kit/SPEC.md` §check-task-names and §Layout and configuration, `native/src/knobs/queue_kit.rs` and `native/src/gates/mod.rs` (the member's knob list) (delta 4).
- `doctrine-kit/DOCTRINE.md`, Absence statements (delta 5).
- `docs/canon-kit/SPEC.md`, `docs/queue-kit/SPEC.md` and `docs/doctrine-kit/DOCTRINE.md`: the generated mirrors, regenerated with `bash gate-sdk/bin/run-gates.sh --emit docs-mirror --write` (deltas 2, 4 and 5).
- `scripts/canon-config.knobs` and `.workflow/audit-roster.txt` (delta 6).
- `.workflow/release-declarations.md` (delta 7).
- `.workflow/prose-bound-ceiling.txt`: the `canon-kit/SPEC.md`, `queue-kit/SPEC.md` and `doctrine-kit/DOCTRINE.md` rows, re-stamped to the counts `check-prose-bounds` prints if deltas 2, 4 and 5 move them (deltas 2, 4 and 5).

The roster came from `grep -n 'PROSE_TELL_GLOBS' canon-kit/SPEC.md native/src/gates/mod.rs native/src/knobs/canon_kit.rs canon-kit/checks/check-prose-tells.gate`, `grep -n 'QUEUE_KIT_PROSE_LEADS' queue-kit/SPEC.md native/src/knobs/queue_kit.rs`, and `grep -n -i 'absence' doctrine-kit/DOCTRINE.md .workflow/audit-roster.txt`.

## Retired spellings

- None — the amendment adds an assertion, an arm and two knobs, and renames nothing.

## Definition of Done

- [ ] **Causal completeness**: every point of canon-kit/SPEC.md §The causal-completeness check holds for G, the placeholder arm and both knobs.
- [ ] **Instruction surfaces: instruction only**: the knob comment and the help lines state what to do; the grounds sit in the two SPEC sections.
- [ ] **Merged with no information lost**: §check-prose-tells reads as one section with seven assertions, and §check-task-names' invariant reads as one sentence.
- [ ] **Amendment deleted**: this file is removed on merge, and `ls SPEC-*.md` at the root lists no file of this amendment.
- [ ] **Removals propagated**: nothing retired.
- [ ] **Gaps filed**: a cross-component gap found during the work is filed with `--emit file-gap`.
- [ ] **The entry moves**: `absence-statement-gate-arms` moves to Done in the landing commit, before the drain stage.
