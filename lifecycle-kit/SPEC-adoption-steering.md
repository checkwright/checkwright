# SPEC amendment: adoption-steering

## What changes

**Mode-framing ruling: reword, no mechanism.** The stage-skill install step
(lifecycle-kit/README.md §Install step 3) and the SPEC's mode list
(lifecycle-kit/SPEC.md §templates/skills/) currently frame binding-shim and
copy-and-fill as a coequal either/or — the SPEC even lists copy first as "the
default". The two have opposite drift postures, so the framing steers:

- **The binding shim becomes the documented default** — it tracks the kit,
  re-vendor reaches it, and `check-skill-binding` + `check-shim-restatement`
  hold it to a thin reference.
- **Copy-and-fill becomes the sanctioned *fork***, presented with its
  consequence inline: you own the ritual prose; upgrades don't reach it; the
  shim gates don't cover it. It is *kept* deliberately — the blessed escape
  hatch that keeps legitimate structural divergence visible and contained,
  and the harness-agnostic floor (the bare-bash upgrade smoke's assumption);
  removing it would drive forks into edits of the vendored template, which
  break Phase-A upgrade determinism with no gate to catch them.

No install-time assembler is built: none exists today, and a mechanism steer
(emit shims by default, copy as explicit opt-out) belongs to whatever
installer the upgrade-path rung's installer question produces — ruled
deferred with that rung, not here.

**Diagnostic ruling: SPEC prose, no instrumentation.** The SPEC's mode
section gains the diagnostic sentence: a consumer reaching for copy to
express *prose* divergence (rather than structural divergence — different
stages, a reshaped machine) signals the slot vocabulary is too thin, and the
fix is richer slots pulling those cases back under shim protection, not more
copying. No gate or telemetry watches for it — which mode a consumer chose
is their tree, not this one.

**Docs-placement ruling: the steer surfaces on the living front door too.**
The generated mirror carries the README/SPEC reframe automatically
(`gen-docs-mirror.sh`), but adoption is decided on the authored living pages,
so `docs/install.md`'s skill-adoption step gains the one-line steer (shim by
default; copy is the sanctioned fork) with the mirror as the behind-link
detail. `docs/lifecycle-kit/index.md` is checked for a mode mention and
aligned only if it already frames the choice.

## Producers and consumers

- **Producer/consumer:** documentation-only — no new state, file, field,
  knob, or gate. The reworded surfaces are produced by this unit's edits and
  consumed by adopting readers; the existing gates that govern those
  surfaces (`check-docs-mirror-fresh` for the mirror, the manifest doc gates
  for README/SPEC/docs pages) are unchanged and re-run green.

## Existing sections updated

- `lifecycle-kit/README.md` §Install step 3 — shim first as the recommended
  default; copy second as the sanctioned fork with its consequence stated.
- `lifecycle-kit/SPEC.md` §templates/skills/ — mode list reordered and
  reworded to match (drops "the default for a consumer whose stages
  diverge" framing for copy; that case is the sanctioned-fork case); gains
  the thin-slot-vocabulary diagnostic sentence.
- `docs/install.md` — one-line steer at the skill-adoption step.
- `docs/lifecycle-kit/SPEC.md`, `docs/lifecycle-kit/README.md` — regenerated
  mirror (`bash scripts/gen-docs-mirror.sh --write`).

## Definition of Done

- [ ] **Causal completeness** — every new state/event/interface has a named,
      reachable producer and a named consumer (none introduced).
- [ ] **Merged with no information lost** — each addition integrated into its
      proper canonical-spec section; the merged doc reads alone.
- [ ] **Amendment deleted** — this file removed on merge; none remain for the
      component.
- [ ] **Removals propagated** — nothing retired; the mirror regenerated.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed
      as debt tasks.
