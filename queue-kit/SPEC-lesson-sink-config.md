# SPEC amendment: lesson-sink-config

## What changes

Harvest-tag sinks stop being tracked literals. The SPEC already rules that a
harvest tag's *sink* is consumer rule content; this repo's close skill
nevertheless hardcodes its staging file — and cannot do otherwise, because
the skill is tracked and naming the downstream repo there would publish a
private path. Two additions close that hole:

- **`bin/lesson-sink.sh <tag>`** — resolves the sink for a harvest tag and
  pipes stdin (the lesson body) into it. Resolution:
  `QUEUE_KIT_LESSON_SINKS` — an associative array, tag → sink command,
  default empty. A missing entry falls open to the default staging append:
  `>> <workflow-dir>/<tag>-harvest.md` (this repo's
  `.workflow/essay-harvest.md` is exactly that default at tag `essay` — the
  current behavior becomes the fallback, keeping its documented reclaim
  path). The sink is a **command, not a path** — ruled: a command lets the
  sink reformat the body into the downstream backlog's own grammar, and the
  plain append case is the default, so the simple consumer configures
  nothing.
- **The local config overlay** — `lib/queue.sh`'s config loader, after
  sourcing the consumer config file, sources `<config>.local.sh` beside it
  when present (gitignored value, tracked name — the
  msg-patterns.local.list precedent: tracking the sink value would itself
  be the leak).

Fail-open is the ruled posture for a missing overlay: a fresh clone must
close cleanly, and the default staging file keeps the honest manual drain
path; the close skill's runtime-artifact step remains the reminder that a
staging file needs reclaiming. A configured sink command that *fails* is a
red close step, not a silent fallback — material must never be half-routed.

## Producers and consumers

- Producer: the consumer close skill's harvest-routing step invokes
  `bash queue-kit/bin/lesson-sink.sh <tag>` with the entry body on stdin.
  The tracked skill now names the mechanism, never the sink value.
- Consumer: the operator's sink command (from the local overlay) or the
  default staging file.
- `QUEUE_KIT_LESSON_SINKS` is read by bin/lesson-sink.sh at resolution
  time; the `<tag>` argument is read at the same transition (lookup key and
  default filename); the body is streamed through, unparsed by the kit.

## Existing sections updated

- §The tag algebra, harvest-tags bullet: sink routing goes through
  bin/lesson-sink.sh and the overlay; the tag names *and* their sinks are
  consumer content, now both with a config home.
- §lib/queue.sh: the loader's overlay-sourcing step.
- §Layout and configuration: the new knob and the `<config>.local.sh`
  convention.
- §templates/: the queue-config template gains the commented knob.
- This repo's consumer edits at build: `.gitignore` entry for
  `scripts/queue-config.local.sh`; `.claude/commands/close.md` harvest table
  cites the helper instead of the literal path.

## Definition of Done

- [ ] **Causal completeness** — every new state/event/interface has a named,
      reachable producer and a named consumer; every new field has a named
      reader at a named transition.
- [ ] **Merged with no information lost** — each addition integrated into its
      proper canonical-spec section (not appended); the merged spec reads as
      one coherent document a reader who never saw the amendment can use
      alone.
- [ ] **Amendment deleted** — this file removed on merge; none remain for the
      component (`ls queue-kit/SPEC-*.md`).
- [ ] **Removals propagated** — grepped every spec for names this change
      retired; nothing dangles.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed
      as debt tasks (a build-time causal gap is resolved that session, not
      deferred).
