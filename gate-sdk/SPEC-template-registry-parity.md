# SPEC amendment: template-registry-parity

Queue entry: **`kit-template-registry-completeness`**.

drift-kit's bundled-KPI claim outruns its shipped registry: `kpi-queue-net-delta`
exists at `drift-kit/kpis/`, is named among the bundled Lead KPIs in
drift-kit/SPEC.md, and is missing from `drift-kit/templates/kpis.list` — the
kit's own install-time registry. Because the kit's smoke copies that template,
the smoke never registers or exercises the KPI, which weakens the kit's
"asserts one row per registered KPI" testing claim.

The entry asked whether such a template is meant to be the **full bundled set**
or a **deliberate starter subset**. This amendment rules: the full bundled set,
and states the predicate that makes the rule safe to generalize.

## What changes

### 1. The shipped-registry rule {design-bearing}

**A kit template that registers artifacts the kit itself ships names every one
of them.** The consumer's *copy* is the consumer's to prune — drift-kit's own
template header already says "delete a line to opt out" — but the shipped
template is the kit's claim about what it bundles, and a kit's SPEC roster, its
README, and its smoke's coverage assertions are all stated over that set. A
starter subset is refused because nothing distinguishes it from an omission: it
is exactly what a dropped line looks like, which is how this instance survived.

**Scope derives from layout, never a roster** — the rule
§check-template-copy-parity already states, applied one axis over. A template
enters the population when `<kit>/templates/<name>.list` has a sibling
**directory** `<kit>/<name>/` holding the artifacts the list registers.
`drift-kit/templates/kpis.list` pairs with `drift-kit/kpis/` and is in. A
template with no such sibling is silently skipped, not failed — the same
silent-skip exclusion the copy-parity gate gives a template with no consumer
counterpart.

### 2. The provenance-seam carve-out, and why it is structural {design-bearing}

The flat form of this rule — *a kit's `templates/` registry must be the full
bundled set* — is **unsafe as stated**. Applied to
`drift-kit/templates/price-table.tsv` it would force a kit literal enumerating
real model ids and prices, which is precisely the provenance seam
(CLAUDE.md §The provenance seam): the per-token prices are public facts, but the
roster of models a consumer runs is that consumer's, and a kit literal
enumerating it would publish it. That template ships schema and placeholder rows
**by design**, and says so in its own header.

The rule is therefore predicated on **registries of things the kit itself
ships** — its own KPI plugins, its own gate names — and the predicate is carried
by the sibling-directory test rather than by a list of exceptions:

- `price-table.tsv` — not a `.list`, and there is no `drift-kit/price-table/`
  directory of kit-shipped artifacts, because its rows are not kit artifacts at
  all. Out of population structurally.
- `gate-sdk/templates/msg-patterns.list` — a `.list`, but there is no
  `gate-sdk/msg-patterns/`: its rows are `grep -E` patterns, consumer rule
  content the kit stubs generically, not names of things gate-sdk ships. Out of
  population structurally.

**This is deliberately not a per-file exception list**, and that is the whole
point of the shape: an exception list re-arms on the next template of this kind,
because someone must remember to extend it, and the failure mode of forgetting
is a kit literal publishing a private vocabulary. Under the sibling-directory
predicate a consumer-rule-content template can never enter the population, since
having no kit-shipped artifacts to register is the same fact as having no
sibling directory of them.

**A second structural consequence, named so build does not get it wrong.**
`drift-kit/templates/kpi-deprecated-surface.sh` is an example plugin shipped
*as a template* for a consumer to adapt; it is not in `drift-kit/kpis/`, so it
is not a bundled artifact and must **not** be required in the registry. The
shipped set is the sibling directory's contents and nothing else.

**Boundary this amendment does not cross.** It rules what a kit's shipped
registry must **contain**. It rules nothing about **where a kit's install entry
point lives** — the duplication between each `smoke/install.sh` and the
installer's recipe, and any per-kit install script that might resolve it,
remains the open question **`kit-owned-install-recipe`** owns, untouched and
un-prejudged here.

### 3. `check-template-registry-parity` — a new gate-sdk gate {design-bearing}

Invariant: for every kit template in the population above, the registry's member
set equals the sibling directory's shipped set, both directions.

Registry members are the non-comment, non-blank lines — the `gates.list`
grammar, which drift-kit's own template header cites. Shipped members are the
basenames of the sibling directory's `*.sh` files, extension stripped, measured
against tracked files (`git ls-files`), so an untracked scratch file in a kit
directory forces nothing. Two assertions:

- **(A) Every shipped artifact is registered** — a bundled plugin absent from
  the template is red. This is the `kpi-queue-net-delta` finding.
- **(B) Every registry line resolves to a shipped artifact** — a line naming no
  file is red, so a deleted plugin cannot leave a line that installs a broken
  registry into every consumer.

Each finding names the kit, the template, and the member. Sweep and config reuse
`gate_kit_roots` / `GATE_SDK_KIT_DIRS` with a positional root override for a
hermetic fixture tree, the sibling meta-gates' shape; **no new knob**.
Fail-closed: an unreadable template, an unreadable sibling directory, an empty
kit roster, or a non-repo cwd with no root argument is a red, never a skip.

### 4. `drift-kit/templates/kpis.list` gains `kpi-queue-net-delta` {mechanical}

One line, in the Lead block beside its siblings, matching the placement the
consumer copy `scripts/kpis.list` already uses. Mechanical because assertion (A)
is the oracle.

### 5. drift-kit/SPEC.md — the templates and smoke claims {mechanical}

The `templates/` section states that `kpis.list` ships the full bundled set and
cites the gate that holds it, so the property is documented where a reader of
that file looks. The smoke's "one row per registered KPI" claim is left as
written and becomes *true of the whole bundled set* once delta 4 lands — the
claim was never wrong, its population was short.

## Producers and consumers

**`check-template-registry-parity` (new gate).**
Producer: the generated pre-commit hook and `gate-sdk/bin/run-gates.sh`, by
registration by name in `scripts/gates.list`. Its `# graph:` manifest couples it
to `*/templates/*.list` and to the sibling artifact directories, so adding a KPI
plugin or editing a template re-fires it; `check-graph` is that manifest's
reader. The generated hook is the artifact a new gate stales — the fan-out and
its regen command are rostered in docs/site-architecture.md §Generated
projections. Consumer: the committing operator, through gate-sdk's output
contract (gate name, file, member), read once at the pre-commit transition.

**`kpi-queue-net-delta` in the shipped registry (new state on an existing
surface).**
Producer: `drift-kit/templates/kpis.list` after delta 4; the enabling
configuration is real rather than test-only, because `drift-kit/smoke/install.sh`
copies that template verbatim into the scratch consumer's gates dir — the
existing copy step, unchanged, is what carries the new line into the smoke.
Consumers, both of which exist today and neither of which needs a change: the
KPI resolver in `drift-kit/lib/`, which reads registry lines and resolves each
against `DRIFT_KIT_KPI_DIRS` then the vendored kits' `kpis/`; and the smoke's
row assertion, which reads the report's rows against the registered set and so
begins exercising this plugin the moment the line exists. No field is added to
any record — a registry line is a bare plugin name in a grammar that already has
one reader.

**The population predicate (new interface between layout and gate).**
Producer: the kit author, by placing a `.list` template beside a same-named
sibling directory — the act of shipping artifacts and a registry for them *is*
the declaration, which is why there is no knob and no marker to forget.
Consumer: the gate's scan at startup, its one reader.

**Whole-component-set reader survey.** The readers of `templates/kpis.list` in
the tree are `drift-kit/smoke/install.sh` (copies it) and, after the copy, the
registry resolver and report assertions that read the consumer-side `kpis.list`;
the readers of `drift-kit/kpis/*.sh` are that resolver and `bin/drift-report.sh`.
`scripts/kpis.list` is this repo's own consumer copy and is **not** in the gate's
population — it is a consumer's pruned copy, deliberately outside the rule. Build
re-runs this survey across the whole tree before writing the gate, without
silencing stderr on any path probe.

## Existing sections updated

- **gate-sdk/SPEC.md §Per-component contracts** — a new
  `### check-template-registry-parity` section: the invariant, the
  layout-derived population, both assertions, the fail-closed posture, and the
  seam predicate with its two structural exclusions (deltas 1–3).
- **gate-sdk/SPEC.md §check-template-copy-parity** — its "scope derives from
  layout, never a roster" paragraph names the sibling gate that applies the same
  principle to `.list` templates, so a reader arriving at either meets both and
  does not read the two `templates/` gates as one overlapping pair. They do not
  overlap: that gate compares a template to a *consumer copy* of itself, this one
  compares a template to the *kit directory it registers*.
- **gate-sdk/README.md** — the gate joins the kit's register-the-gates block
  between its `gate-roster:` markers, which `check-readme-roster` asserts against
  the shipped `checks/` basenames; a gate landed without that line is red.
- **drift-kit/SPEC.md §templates/** — `kpis.list` documented as the full bundled
  set, citing the gate (delta 5).
- **drift-kit/templates/kpis.list** — the missing line (delta 4).
- **docs/site-architecture.md §Generated projections** — the new gate's fan-out,
  per that section's standing roster; named here so build treats it as claimed
  work rather than an orphan a batch adopts on its own authority.

## Definition of Done

- [ ] **Causal completeness** — every new state/event/interface has a named,
      reachable producer and a named consumer; every new field has a named
      reader at a named transition.
- [ ] **Merged with no information lost** — each addition integrated into its
      proper canonical-spec section (not appended); the merged spec reads as one
      coherent document a reader who never saw the amendment can use alone.
- [ ] **Amendment deleted** — this file removed on merge; none remain for the
      component (`ls gate-sdk/SPEC-*.md`).
- [ ] **Removals propagated** — grepped every spec for names this change
      retired; nothing dangles.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed as
      debt tasks (a build-time causal gap is resolved that session, not
      deferred).
