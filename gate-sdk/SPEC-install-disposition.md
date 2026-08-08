# SPEC amendment: kit-owned-install-recipe

A gate declares **its own install disposition**, on the gate, beside its
`# graph:` and `# spec:` directives. `recipe_gates`' per-kit case statement is
then **derived and deleted** rather than maintained — the installer stops
carrying a second copy of a fact each kit already owns.

## The filed mechanism was weighed against the tree and refused

The queue entry's costed fix was: push a `bin/install-<kit>.sh` down into each
kit, have `smoke/install.sh` delegate to it and the installer's `init` call it.
That rests on the premise that the zero-config subset is *one fact encoded
twice*. **A census of all eleven kits falsifies the premise** — recorded at
`.workflow/survey-record.md`, this iteration, with its witness.

The two rosters describe **two different trees**. Each kit's `smoke/install.sh`
registers against a scratch consumer *the smoke script itself builds and seeds*;
`recipe_gates` registers against the tree `init` makes. In every gated kit the
installer's arm is a strict subset of the smoke's — the difference direction
"in recipe but not smoke" is **empty everywhere** — and the difference is
deliberate where it is largest. lifecycle-kit registers **zero** gates at
install because its gates read a stage attestation only a stage session can
write, and its smoke registers all seven because the smoke stamps that
attestation for real. site-kit registers zero at install and five in smoke
because the smoke writes `docs/CNAME` and a workflow template that `init` never
writes.

So a single entry point returning one roster cannot serve both callers: it would
either arm lifecycle-kit's gates on an adopter tree with no attestation, or
strip the smoke's coverage down to the installer's subset. **The refusal is of
the mechanism, not of the unit** — the exposure the entry names is real and
unchanged: a kit that adds a zero-config gate the installer never learns about
ships to adopters unregistered and silent, and nothing reds.

What must be kit-owned is therefore the **disposition**, not a roster; each
caller derives its own set from the tree it actually made.

## What changes

**E1 — a new per-gate directive, `# install: <disposition>`.**
`{design-bearing}`
One line in each gate's header block, beside `# graph:` and `# spec:`, and in a
`.gate` descriptor on the same terms — a ported gate is still a gate a kit
ships. Three values, and the vocabulary is closed:

- **`zero-config`** — the gate reads a surface `init` itself writes, so it
  registers in a fresh consumer.
- **`on-surface`** — the gate's subject is one the adopter authors later (a
  glossary, a docs host, a stage attestation), so it arms when that surface
  exists rather than at install.
- **`never`** — the gate is not auto-registered on any tree. This is the class
  §Consumer smoke's declaration valve already recognises from the other side.

The vocabulary names **install-time reachability**, never a path, a surface
name, or a project's own governed-file vocabulary. That is deliberate: a
disposition enumerating consumer paths would be a kit literal carrying one
project's tree layout, which the provenance seam forbids. The path a gate reads
stays where it already lives — the gate's own configured knob.

**E2 — `recipe_gates`' case statement is deleted and the function derived.**
`{design-bearing}`
`recipe_gates(kit, profile)` becomes: every `checks/` member of that kit — both
declaration spellings, `check-*.sh` and `check-*.gate` — whose `# install:`
disposition is `zero-config`. Nine literal gate-name lists and the arms that
hold them go; the `lifecycle-kit) : ;;` arm goes too, and its reason survives
where it belongs, as `on-surface` on each of that kit's seven gates. The
profile argument E1 does not yet vary on is `profile-keyed-install`'s seam and
stays as that amendment leaves it.

**E3 — `recipe_needs_queue` and `recipe_needs_agent_file` are unchanged, and the
non-extension is ruled.** `{design-bearing}`
Both are kit-name case arms and both look like the same defect. They are not:
they answer *what a kit's install must seed*, not *what it may register*, and
the seeded surface is a property of the kit rather than of any one gate — so a
per-gate directive has nothing to say about it. Recorded so a build session
finds the boundary drawn rather than extending the mechanism into a place it
does not fit.

**E4 — `check-install-disposition`, a new gate-sdk meta-gate with its
`good/`+`bad/` fixture pair.** `{design-bearing}`
Three assertions over every kit root in `gate_kit_roots`:

- **(A) Declared** — every `checks/` member carries exactly one `# install:`
  line with a value from E1's set. A gate that ships without one is red; this is
  the assertion that closes the live exposure, because a newly added zero-config
  gate cannot reach an adopter undeclared.
- **(B) Smoke superset** — every `zero-config` member of a kit appears in that
  kit's `smoke/install.sh` roster. The direction is sound and is what the census
  measured: the smoke's tree is a superset of the tree `init` makes, so a gate
  the installer registers must be registrable there too. The converse is **not**
  asserted, and that is the census's finding made mechanical.
- **(C) No second copy** — `installer/lib/common/recipe.sh` carries no literal
  gate name. The de-literalization holds going forward rather than only at the
  commit that lands it.

Fail-closed on a non-repo cwd, an empty roster, or an unreadable header, the
sibling meta-gates' shape; config reuses `GATE_SDK_KIT_DIRS`, no new knob.

**E5 — the disposition is declared across every shipped gate.** `{mechanical}`
One directive line per `checks/` member across all eleven kit roots. The values
are not free choice: `zero-config` is exactly the set `recipe_gates` registers
today, `on-surface` is what a kit's smoke registers and the installer does not,
and `never` is the already-declared unregistered class. The sweep is a
transcription of two rosters the tree already carries into one place, verified
by E4 running green with `recipe_gates` derived — a rename/merge sweep against a
fixed oracle.

**E6 — gate-sdk/SPEC.md gains §The install disposition, and §Consumer smoke's
per-kit contract cites it.** `{design-bearing}`
The new section owns E1's vocabulary, E2's derivation, and the two-tree ground.
§Consumer smoke's `smoke/install.sh` clause today says *which* gates it registers
is not the author's discretion and points at the registration accounting; it
gains the second half — that the installer's subset is derived from the same
declarations, and that the smoke's roster is the superset E4(B) holds it to.
The kit-landing checklist gains the declaration as a clause.

**E7 — `installer/README.md` §What init seeds loses its filed-work paragraph.**
`{mechanical}`
Its *honest limit* names the second encoding and says collapsing the two into
one per-kit source is filed as queued work. On merge that work has landed, so
the paragraph is rewritten to state what is true after: the starting roster is
derived from each gate's declared disposition, and the smoke's richer roster is a
superset by contract rather than by coincidence.

## Producers and consumers

**`# install: <disposition>`** — Producer: the gate author, at the moment a gate
is written; enforced reachable by E4(A), so the declaration cannot be omitted
rather than merely being expected. It ships to consumers with the gate: it is a
header directive on a file already in the payload, and for a ported gate it
rides the `.gate` descriptor, which §Consumer payload already commits to
shipping.

Consumers, three, each by a named mechanism:

- **`recipe_gates`** in `installer/lib/common/recipe.sh`, reading the vendored
  kit's `checks/` headers in the payload at `init` time, at the transition where
  `plan_gates` resolves a fresh consumer's `gates.list`. This is the live
  install path, not a test path.
- **`check-install-disposition`**, by header scan across `gate_kit_roots`, at
  pre-commit and in the battery.
- **`bin/run-consumer-smoke.sh`'s registration accounting**, which already
  probes every shipped-but-unregistered gate and derives an exemption from its
  exit code. It gains a corroborating read: a gate declaring `zero-config` that
  the probe finds surface-absent in the scratch consumer is a **contradiction**
  between a declaration and a derivation, and is reported as such rather than
  silently exempted. This is the arm that keeps a wrong declaration from being
  as invisible as a missing one.

**No new field is added to `checkwright.lock`.** The disposition is a property of
the gate, read from the payload at install time; recording it in the consumer's
manifest would put a second copy in the one place that outlives the payload.

## Existing sections updated

- **gate-sdk/SPEC.md §Consumer smoke** — E6: the `smoke/install.sh` clause, the
  registration accounting's new contradiction verdict, and the kit-landing
  checklist clause.
- **gate-sdk/SPEC.md, new §The install disposition** — E1, E2, E4's contract.
- **gate-sdk/SPEC.md §check-install-disposition** — E4's own per-component
  section, the meta-gate roster's shape.
- **installer/README.md §What init seeds** — E7: the *honest limit* paragraph.
- **Each kit README's `<!-- gate-roster:begin -->` block** — untouched. It holds
  name-set parity with `checks/` under `check-readme-roster`, which reads only a
  line's first `check-`-prefixed token; the disposition is not added there,
  because that block is prose an adopter pastes and a machine-read annotation
  inside a pasted snippet is a second grammar in a surface that has one.

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
      retired; nothing dangles. Specifically: no literal gate name survives in
      `installer/lib/common/recipe.sh`, and no surface still describes the
      installer's roster as a second encoding.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed as
      debt tasks (a build-time causal gap is resolved that session, not
      deferred).
