# SPEC amendment: artifact-verification

## What changes

**Home ruling: extend Oracle-first (no new numbered rule)** — the gap is an
oracle gap (the battery lints the tree; the artifact is a second oracle the
rule never named), and a new rule would force a renumber the gap-disposition
amendment already spends once. Oracle-first gains the artifact clause, three
sharpenings the docs-site-chrome work paid for:

1. **Gate-green is tree-correct, not artifact-correct.** A change whose real
   output is a deployed or generated artifact (a rendered site, a compiled
   binary, a published package) *names the artifact surface it carries* and
   exercises the artifact before the task is done — never inferred from a
   clean battery. (The motivating instance: a link-extension bug 404'd every
   kit page of the rendered site while all gates ran green.)
2. **Reachable includes cheap-to-stand-up.** An absent-but-installable
   runtime is not unreachable; one install command away counts as reachable
   for the build stage's run-the-system duty.
3. **A local replica must be deployment-faithful** — toolchain version- and
   plugin-matched (the pinned deployment toolchain is the honest oracle); a
   newer local toolchain both invents failures the deployment never shows and
   masks real ones it would.

**Build-template ruling.** The run-the-system guidance in lifecycle-kit's
`templates/skills/build.md` gains one line: a change carrying an artifact
surface names it and exercises the artifact, deployment-faithfully, before
the stage exits. Widest-true-tier: true for every consumer running the
lifecycle, so it lands in the kit template, not this repo's shim.

**Ruled out.** A bootstrap step in the harness /verify skill — the recipe's
widest true tier is the kit template plus the rule text; a harness-specific
skill narrows it. Also ruled out: any gate — "did the session exercise the
artifact faithfully" is not machine-decidable (the Enforcement-first
carve-out); the duty is event-keyed by the change itself (an artifact-bearing
diff is its own trigger), so it needs no audit-roster entry either.

## Producers and consumers

- **Producer:** the build session, at the point a change touches a surface
  that feeds a deployed or generated artifact; the template line and the rule
  text are the triggers (build sessions load the build skill; the rule loads
  via the doctrine link).
- **Consumer:** the validate stage inherits an artifact already exercised;
  no new state, file, or field is introduced — the clause changes session
  behavior, not machine surface.

## Existing sections updated

- `doctrine-kit/DOCTRINE.md` — Oracle-first (§Methodology-maintenance rules)
  gains the artifact clause; its *Under agent work* note gains the
  faithful-replica sentence.
- `lifecycle-kit/templates/skills/build.md` — run-the-system guidance gains
  the artifact-surface line (template change; consumer shims inherit it by
  reference, unchanged).
- CLAUDE.md digest: the Oracle-first bullet's one-line wording is re-derived
  via `install-doctrine.sh` if the digest line changes; keep the digest line
  stable if the clause fits behind the pointer.

## Definition of Done

- [ ] **Causal completeness** — every new state/event/interface has a named,
      reachable producer and a named consumer; every new field has a named
      reader at a named transition.
- [ ] **Merged with no information lost** — each addition integrated into its
      proper canonical-spec section; the merged doc reads alone.
- [ ] **Amendment deleted** — this file removed on merge; none remain for the
      component.
- [ ] **Removals propagated** — nothing retired by this change.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed
      as debt tasks.
