# SPEC amendment: slug-liveness

## What changes

Living prose may name queue tasks, and nothing couples the mention to the
live queue — the public orchestration page kept calling a landed task
deferred for a full iteration. The class is mechanically decidable, so it
gets a gate, not a duty.

**Mention grammar (kit mechanism).** A bold-code token — `` **`<token>`** ``
— whose token matches the task-slug grammar (`check-task-names` owns it;
kebab-case, alphanumeric start, so `--flag` mentions fall outside) is the
sanctioned way prose *claims queue membership*. Prose about landed work
drops the bold-code form and cites the owning SPEC instead — the living-page
doctrine's current-state rule, now with a grammar a gate can read.

**Gate.** New `check-queue-slug-liveness` (queue-kit/checks/): for every
file matched by new consumer knob `QUEUE_KIT_PROSE_SURFACE_GLOBS` (default
empty — which surfaces make queue claims is consumer config, the provenance
posture; same knob shape as the template-governance amendment's
`CANON_KIT_PROSE_SURFACE_GLOBS`), every slug-shaped bold-code token must
resolve against the queue file's live slug set (`lib/queue.sh` already
parses it). A dead claim fails naming file, line, and slug. Skeleton
contracts apply: `good/`+`bad/` fixture pair, `# graph:` manifest coupling
the knob's surfaces to the queue file, registration in `gates.list`,
regenerated hook/graph/enforcement artifacts.

**This repo's config.** `scripts/queue-config.sh` sets the globs to
`docs/*.md` — the living pages; `docs/posts/` (immutable) and the generated
`docs/<kit>/` mirror sit outside the glob by shape. Verified single live
instance today: the orchestration page's one remaining bold-code slug
resolves; the two `--flag` bold-code mentions in mirrored SPECs are outside
both the glob and the slug grammar.

## Producers and consumers

- **Producer:** the gate, at every battery run; reachable via `gates.list`
  registration and the regenerated pre-commit hook.
- **Consumer:** the battery/operator (a red run names the stale claim); the
  docs author, for whom the bold-code form is now a checked contract rather
  than a typographic habit.
- **Knob reader:** `QUEUE_KIT_PROSE_SURFACE_GLOBS`, read by the gate alone;
  documented in queue-kit/SPEC.md §Layout and configuration's roster with
  its empty default and the claim-grammar semantics.

## Existing sections updated

- `queue-kit/SPEC.md` — new §check-queue-slug-liveness contract; §Layout and
  configuration gains the knob; §The tag algebra gains one sentence naming
  the bold-code mention grammar as the prose-side claim form.
- `scripts/queue-config.sh` — this repo's surface globs.
- `scripts/gates.list`, generated pre-commit hook, `docs/check-graph.html`,
  `docs/enforcement.md` — regenerated for the new gate.
- `queue-kit/README.md` — the gate joins the Install list; mirror
  regenerated.

## Definition of Done

- [ ] **Causal completeness** — every new state/event/interface has a named,
      reachable producer and a named consumer; every new field has a named
      reader at a named transition.
- [ ] **Merged with no information lost** — each addition integrated into its
      proper canonical-spec section; the merged doc reads alone.
- [ ] **Amendment deleted** — this file removed on merge; none remain for the
      component.
- [ ] **Removals propagated** — nothing retired.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed
      as debt tasks.
