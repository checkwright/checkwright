# SPEC amendment: template-conformance

Root-level (a cross-kit workflow ruling with no single owning component).
Merge target: gate-sdk/SPEC.md §Consumer smoke. Resolves the deferred task
`queue-starter-template-not-spec-kit-clean`.

## What changes

**The starter-template conformance contract goes battery-wide.** The
queue-kit iteration's lesson established that a kit's starter template must
pass *that kit's own* gates when copied verbatim. This amendment extends
it: a shipped starter template must pass the **full battery** — every
Checkwright kit's gates — when copied verbatim into a combined-kit
consumer. Kits are designed to compose; the first combined tree is where a
per-kit-clean template still reddens a foreign kit's gate. Attested
instance: queue-kit's `templates/TASK-QUEUE.md` reddens spec-kit's
`check-amendment-queue` — its New-Features teaching entries carry no
`[spec:]` tag, and its `spec-ready-entry` example both sits `[spec:]`-ready
in the deferred section (a promote finding) and dangles a ref at a
nonexistent `SPEC-example.md`.

The ruled-out alternative: documenting that a spec-kit consumer *fills*
the template rather than copying it verbatim. Rejected — it contradicts
the recorded lesson, makes red-on-first-install the documented onboarding
experience for combined trees, and permanently entrenches the smoke
harness's workaround (writing a minimal queue instead of exercising the
shipped template). Also ruled out: shipping a paired `SPEC-example.md` so
the teaching refs resolve — it plants a fake live amendment in every
consumer tree, violating spec-kit's amendments-are-short-lived invariant.

### Template deltas (queue-kit `templates/TASK-QUEUE.md`)

- The top-level-entry teaching examples (`example-feature`,
  `downstream-feature`, `example-subtask` and the prose-note bullet) move
  from `## New Features` to `## Technical Debt` — an active section with
  identical grammar (the template already says so) on which
  `check-amendment-queue` imposes no `[spec:]` requirement. `## New
  Features` keeps only its indented teaching prose.
- `spec-ready-entry` (the `[spec: SPEC-example.md]` example) is deleted;
  the `[spec:]` tag shape is taught in an indented prose note instead.
  Constraint on that prose: the pairing scan is file-wide, so no line in
  the template may spell a bracketed *non-empty* spec ref — the bare
  `[spec:]` literal is safe (the ref extractor requires a non-empty ref),
  a bracketed placeholder filename is not.
- `example-deferred` `[needs-spec]` stays — already clean under both kits.

## Producers and consumers

Enforcement producer: queue-kit's `smoke/install.sh` switches from writing
a minimal filled queue to **copying `templates/TASK-QUEUE.md` verbatim**.
Consumer: `gate-sdk/bin/run-consumer-smoke.sh`, which asserts the full
battery green on the installed tree — from this change on, a template
regression against *any* kit's gates reddens the harness instead of
waiting for a hand-run validate proof. The contract is thereby mechanical,
not ritual.

## Existing sections updated

- gate-sdk/SPEC.md §Consumer smoke gains the contract sentence (templates
  ship battery-clean verbatim; the smoke installs exercise them verbatim
  where one exists).
- queue-kit/SPEC.md §templates/ cites the contract (star topology — the
  owner is gate-sdk's §Consumer smoke).
- This repo's Lessons-Learned entry on starter-template conformance is
  superseded by the spec text and is retired at this iteration's close
  (its harvest path).

## Definition of Done

- [ ] **Causal completeness** — the contract has a mechanical producer
      (verbatim template copy in queue-kit's smoke install) and a named
      consumer (`run-consumer-smoke.sh` battery assertion).
- [ ] **Merged with no information lost** — contract into gate-sdk/SPEC.md
      §Consumer smoke; citation into queue-kit/SPEC.md §templates/.
- [ ] **Amendment deleted** — this file removed on merge; `ls SPEC-*.md`
      clean at root.
- [ ] **Removals propagated** — the superseded Lessons-Learned entry
      retired; no spec still describes the minimal-queue smoke install.
