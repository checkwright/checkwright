# SPEC amendment: lesson-channels

One mechanism closes three queue entries — `lesson-disposition-traceability`,
`lesson-pub-harvest`, `lesson-context-tag` — because all three are the same
thing: a tag vocabulary on the Lessons-entry lead line, with routed handling.
The `## Lessons Learned` section is today a write-only buffer between the
filing session and the close session: no gate holds the close ritual's
per-entry disposition mandate, no later session of the iteration sees an
entry it did not write, and communication-worthy prose is dispositioned into
rules and lost. The three entries merge this amendment together — one build
unit.

## What changes

### Lesson lead-line tags (syntax: queue-kit)

A Lessons entry is a top-level bullet under `## Lessons Learned`; tags are
square-bracket literals on its **lead line** (the queue-wide doctrine — the
lead line is the only line the parsing tools scan). Two classes:

- **`[attend]`** — fixed spelling, kit mechanism (the inbound channel): the
  filing session marks a lesson as a live point of attention for later
  sessions *of the same iteration*.
- **Consumer harvest tags** — `QUEUE_KIT_LESSON_TAGS` (new queue-kit knob,
  array of bare tag names, default empty): the outbound vocabulary. The tag
  names, their sinks, and their handling are consumer rule content — the
  kit ships no tag name, no sink, no sink content (the
  `check-graph`/`graph-vocab` seam). The slugged motivation says `pub`, but
  a tag may serve any communication purpose (essay, talk, internal
  knowledge sharing) — hence a vocabulary, not a single literal.

`check-tag-lead-line` widens its governed tag set with `[attend]` plus the
configured lesson tags: a lesson tag off the lead line is silently unread
by every reader below, exactly the no-op that gate exists to catch.

### Inbound: session-context injection (queue-kit → hook)

`queue-index.sh` appends an **attention block** to both its default and
`--collapse-deferred` output when the current iteration's Lessons section
carries `[attend]` entries: each entry's lead line, capped at
`QUEUE_KIT_ATTEND_CAP` entries (new knob, default `3`), overflow noted as
`(+N more [attend])`. Lead lines only, never bodies. Because the
session-context hook already embeds `queue-index.sh` output, the injection
reaches every later session with **zero consumer-hook edits** — the causal
chain is filing session tags → queue-index emits → hook shows.

The injected surface is always-loaded, so the cap is the budget:
`always-loaded.sh` already meters the hook body via
`CONTEXT_KIT_HOOK_CMD`, and the close-stage brevity pass sees any growth.

**Boundary death is mechanical, not hoped for.** A lesson that outlives
its close becomes the standing per-session instruction
drift-kit/templates/close-knowledge.md forbids. Two enforcers: close's
exit condition clears the section, and `enter-stage.sh scope` — the
iteration-boundary reset — now **refuses when `## Lessons Learned` is
non-empty** (same refusal contract as its `check-stage-entry` precondition),
so an untriaged lesson cannot cross into the next iteration and no
`[attend]` injection can outlive its iteration.

### Outbound + traceability: dispositions leave evidence (lifecycle-kit)

The close ritual's disposition set becomes **rule / task / harvest /
discard** (fixed vocabulary, mechanism; `harvest` is the new outbound
disposition — a tagged entry's body is appended to the sink the consumer's
close skill names for that tag). Measured need (a larger private consumer
of this lifecycle): 42 close commits cleared ~180 lesson bullets and 12
commits left no trace in tree or message — discard-by-omission under
close-stage fatigue.

Evidence home ruling: **a stamped file, not the commit body** — the
battery runs at pre-commit, when no commit message exists yet, so only a
file is mechanically decidable (the `check-stage-evidence` fail-closed
precedent: the claim must leave evidence). New governed surface
`LIFECYCLE_KIT_LESSON_EVIDENCE_FILE`, default
`${GATE_SDK_WORKFLOW_DIR:-.workflow}/lesson-evidence.txt`: a `# contract:`
header (the `validate-evidence.txt` pattern), then one line per
disposition —

```
<iteration> lesson <rule <file> | task <slug> | harvest <tag> | discard <reason>> — <lead-line prefix>
```

`enter-stage.sh scope` boundary-truncates it back to its header alongside
the other evidence files (git history keeps the stamps; `trajectory.sh`
may harvest them later — out of scope here).

New gate `lifecycle-kit/checks/check-lesson-disposition.sh`: every Lessons
entry present at HEAD and absent from the worktree must match a
disposition line (lead-line-prefix match) in the evidence file; count and
per-entry matching both hold, fail-closed on an unreadable surface.
Diffing HEAD against the worktree is fixture-hostile (the
`check-task-conservation` precedent), so the gate takes optional
`[queue-head] [queue-worktree] [evidence-file]` override args and the
fixture pair exercises it hermetically (the `check-trajectory-fresh`
synthetic-args precedent). Shape validation only: `harvest <tag>` is not
validated against `QUEUE_KIT_LESSON_TAGS` (that would cross-couple the
kits' configs; the close skill and tag-lead-line hold the vocabulary).

The close-skill template (`templates/skills/close.md`) step 1 gains the
disposition-stamp instruction and the harvest route; this repo's
`.claude/commands/close.md` gains the concrete routing table. A
private-seam lesson (BRIEF.local.md) stamps as
`discard private-seam (BRIEF.local.md)`.

### This repo's consumer config (the reachable-producer requirement)

A configured route must exist somewhere real, or the outbound mechanism is
dead outside unit tests. This repo configures one tag now:
`QUEUE_KIT_LESSON_TAGS=(essay)` (new `scripts/queue-config.sh` — the repo
runs queue-kit on defaults today), routed by `.claude/commands/close.md`
to `.workflow/essay-harvest.md` — gitignored operator material feeding the
`launch-comms` methodology essay; reclaim path: merged into the essay,
then cleared (the Housekeeping rule for gitignored artifacts).

## Producers and consumers

- `[attend]` tag — producer: any stage session filing a lesson (typically
  build); consumers: `queue-index.sh` (emits the attention block at every
  hook run), `check-tag-lead-line` (placement).
- Harvest tags — producer: the filing session; consumers:
  `check-tag-lead-line` (placement), the close session's triage (routes
  body → sink per its skill's table), `check-lesson-disposition` (shape of
  the `harvest <tag>` evidence line).
- `lesson-evidence.txt` lines — producer: the close session at triage
  (each field: disposition kind + ref read by `check-lesson-disposition`
  at the commit-time diff transition; the lead-line prefix is the join key
  to the removed entry); second consumer: `enter-stage.sh scope`
  (boundary-truncation).
- `QUEUE_KIT_LESSON_TAGS` / `QUEUE_KIT_ATTEND_CAP` — read by queue-kit's
  loader (`lib/queue.sh`) at every gate/index invocation; this repo's
  `scripts/queue-config.sh` sets the tag list, so the config path is
  exercised by a deployed configuration from day one.
- The scope-entry Lessons-empty refusal — producer: `enter-stage.sh`;
  consumer: the scope session operator (refusal message names the
  untriaged entries).

## Existing sections updated

- queue-kit §The tag algebra: gains the lesson-tag paragraph — `[attend]`
  fixed, harvest vocabulary as the *one* configured exception to
  "fixed spelling", with the seam rationale (consumer channels are rule
  content, so their names cannot be kit literals).
- queue-kit §bin/queue-index.sh: the attention block and its cap.
- queue-kit §check-tag-lead-line: governed set widens.
- queue-kit §Layout and configuration: the two knobs.
- lifecycle-kit §bin/enter-stage.sh: scope-boundary truncation of the
  lesson-evidence file + the Lessons-empty refusal.
- lifecycle-kit §Per-component contracts: new §check-lesson-disposition.
- lifecycle-kit §templates/skills/: close template's disposition step.
- CLAUDE.md battery list is untouched (the new gate registers by name in
  `scripts/gates.list`; lifecycle-kit fixtures already run).

## Definition of Done

- [ ] **Causal completeness** — every new state/event/interface has a named,
      reachable producer and a named consumer; every new field has a named
      reader at a named transition.
- [ ] **Merged with no information lost** — each addition integrated into its
      proper canonical-spec section (not appended) across queue-kit and
      lifecycle-kit; each merged spec reads as one coherent document.
- [ ] **Amendment deleted** — this file removed on merge; none remain for the
      component (`ls lifecycle-kit/SPEC-*.md queue-kit/SPEC-*.md`); all three
      queue entries move to Done together.
- [ ] **Removals propagated** — grepped every spec for names this change
      retired; nothing dangles.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed as
      debt tasks (a build-time causal gap is resolved that session, not
      deferred).
- [ ] **Fixture pairs** — check-lesson-disposition good/bad (hermetic via
      override args); queue-index attend block + cap in the index-test corpus;
      tag-lead-line bad fixture with an off-lead lesson tag.
