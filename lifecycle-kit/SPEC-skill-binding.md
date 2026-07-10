# SPEC amendment: skill-binding

## What changes

The stage-skill templates gain a second consumption mode,
**consume-by-reference**, beside the existing copy-and-specialize: a
consumer skill may be a thin **binding shim** that instructs the session to
read the kit template and apply the consumer's slot-fills, making the
templates the executed surface rather than prose their own vendor never
runs. One owner per layer: generic doctrine lives only in the template;
consumer rule content lives only in the shim's bindings.

Three new names:

- **Named slots (template grammar).** Each free-prose consumer placeholder
  `*<…>*` in a stage template becomes a named slot:
  `*<slot-name: guidance prose>*` — slot name `[a-z][a-z0-9-]*`, unique
  within its template, followed by `:` and the guidance the copy-editor or
  shim author replaces. Copy-and-specialize consumers are untouched by the
  naming: they still overwrite the whole `*<…>*` span.
- **Binding directive + `## Bindings` (shim grammar).** A shim is a skill
  file whose body opens with a directive line naming its template —
  `Execute the template at <repo-relative path>, applying the bindings
  below.` — followed by a `## Bindings` section holding exactly one entry
  per template slot: `**slot-name** — <consumer content>` (multi-line
  content indents under its lead line). Nothing else: doctrine restated
  from the template in a shim is the defect this amendment exists to
  remove.
- **`check-skill-binding` (gate, lifecycle-kit).** Invariant: every skill
  under `LIFECYCLE_SKILLS_DIR` that carries a binding directive (a) names a
  template file that exists, and (b) binds exactly the template's slot set —
  an unbound slot is red, an orphan binding naming no slot is red. Files
  without a directive (copy-and-specialize skills, non-stage skills) are
  not read. Ships as a check-skeleton copy with a `good/`+`bad/` fixture
  pair, registered in `scripts/gates.list`; its `# graph:` manifest couples
  the templates dir and the skills dir.

Consumer conversion in the same unit: this repo's five stage skills
(`.claude/commands/{scope,align,build,validate,close}.md`) become shims.
The conversion starts with the template-vs-command diff audit: generic
doctrine found only in a command is back-ported into the template first
(the audit already named build's question triage, align's read-site
verification, close's already-filed body check as template doctrine that
never ran); what remains in each command must be expressible as slot
bindings, and a remainder that is neither doctrine nor a slot-fill forces
the placeholder question back to design — every slot a shim needs must
exist as a template placeholder before the shim lands.
`release-sweep.md` stays copy-and-specialize (no consumer copy exists
here); `agent-execution.md` is not a lifecycle template and is out of
scope.

## Producers and consumers

- **Slot** — producer: the template author (kit); consumers: the shim's
  `## Bindings` entry (must bind it — enforced by `check-skill-binding`)
  and the executing stage session, which reads the template and substitutes
  the binding at the slot site.
- **Binding directive** — producer: the consumer shim; consumers: the stage
  session (follows it to the template) and `check-skill-binding` (uses it
  to select which skills it governs).
- **`check-skill-binding`** — producer: registered in `gates.list`,
  coupled into the pre-commit hook via its `# graph:` manifest (regenerate
  hook + CHECK-GRAPH.html + enforcement map on landing); consumer: the
  battery (`run-gates.sh`) and the fixture runner. Config it reads:
  `LIFECYCLE_SKILLS_DIR` (exists today, default `.claude/commands`) — no
  new knob.

## Existing sections updated

- `lifecycle-kit/SPEC.md` §templates/skills/: currently specifies
  copy-and-specialize only ("Structure is copied, not imported"). Rewritten
  to own both modes: copy-and-specialize (self-contained, legible, the
  default for a consumer that diverges) and consume-by-reference (the shim
  grammar above, for a consumer that tracks the kit — this repo dogfoods
  it). The slot grammar lands here; the sentence claiming copy is the only
  mode is retired.
- `lifecycle-kit/SPEC.md` §Per-component contracts: gains the
  `check-skill-binding` contract section.
- The five stage templates: placeholders renamed to named slots; back-ported
  doctrine integrated in place.
- `CLAUDE.md` / repo README touch only if the shim conversion changes what a
  contributor must know (expected: none — the stage ritual is unchanged, its
  source of truth moves).

## Definition of Done

- [ ] **Causal completeness** — every new state/event/interface has a named,
      reachable producer and a named consumer; every new field has a named
      reader at a named transition.
- [ ] **Merged with no information lost** — each addition integrated into its
      proper canonical-spec section (not appended); the merged spec reads as one
      coherent document a reader who never saw the amendment can use alone.
- [ ] **Amendment deleted** — this file removed on merge; none remain for the
      component (`ls lifecycle-kit/SPEC-*.md`).
- [ ] **Removals propagated** — grepped every spec for names this change
      retired (the copy-only claim; doctrine sentences deleted from shims);
      nothing dangles.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed as
      debt tasks (a build-time causal gap is resolved that session, not
      deferred).
