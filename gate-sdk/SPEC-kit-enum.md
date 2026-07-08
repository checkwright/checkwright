# SPEC amendment: kit-enum

Closes the hand-maintained-kit-set drift axis: the kit roster silently
drifted as later kits landed (drift-kit fell out of the delegation meta
paths and the whole-tree gates' couples; context-kit's gate is still
uncoupled from the gate-family meta-gates). `gate_kit_roots` is the
canonical enumeration; every other list of kits is either derived from it
or gated against it. Derivation is preferred wherever the kit owns the
surface — a deleted drift axis beats a gated one; one residual meta-gate
covers what derivation cannot reach (a hand-list a future gate author
writes anyway).

## What changes

- **`kit:` couples token** — the `# graph:` manifest grammar gains one
  couples form, `kit:<glob>`, which the shared manifest reader in
  `lib/gate.sh` expands to `<kit-root>/<glob>` (repo-root-relative) for
  every `gate_kit_roots` member at read time. Whole-tree gates
  (check-shellcheck, check-comment-tier, check-spec-pointer, the
  gate-family meta-gates) migrate their per-kit hand lists to the token;
  non-kit couples (`scripts/*.sh`, `.workflow/*.txt`, `scripts/gates.list`)
  stay literal. Expansion over-approximates by design: a kit whose files
  match the glob is coupled even where the gate's subject is narrower —
  an extra trigger runs a green gate, while a missing one skips a red.
- **Meta-paths auto-union** — `delegation-kit/lib/delegation.sh`, after
  loading consumer config, appends every `gate_kit_roots` member (as a
  root-relative `dir/` prefix) to `DELEGATION_KIT_META_PATHS` when
  `gate.sh` is resolvable: a vendored kit's edits are meta-layer by
  definition, so the consumer no longer hand-lists kit dirs (this repo's
  `scripts/delegation-config.sh` drops its kit roster and keeps only the
  non-kit prefixes). Without `gate.sh` the config is used as written —
  the union is additive, never a filter, so a consumer cannot lose a
  prefix it declared.
- **`check-kit-enum`** (gate-sdk/checks, tier=precommit) — the residual
  meta-gate for hand-lists derivation cannot reach: for every registered
  gate, a `couples=` set that literally names two or more `gate_kit_roots`
  members with a common glob suffix must name every kit root having
  tracked files matching that suffix — the fix the help text names is the
  `kit:` token, not a longer hand list. Fail-closed: an unreadable
  manifest or unresolvable registered gate is a red, not a skip. Ships
  with a `good/`+`bad/` fixture pair per the four gate contracts.

## Producers and consumers

- Token: produced by gate authors in `# graph:` lines; consumed by the
  `lib/gate.sh` manifest reader, which `gen-pre-commit.sh` (hook
  emission), `check-graph` (freshness + the HTML projection), and
  `check-kit-enum` share. The generated hook inlines the *expanded*
  globs, so a kit added later reddens `check-graph` (committed hook ≠
  `--emit` output) until regeneration — the freshness gate is the
  forcing function that keeps the static hook honest, and it runs in
  every full battery.
- Auto-union: produced by `delegation.sh` at config load; consumed by
  `check-gate-tamper`'s meta/product split — the same run that loads the
  config sees the unioned array, no new file or knob.
- `check-kit-enum`: registered in this repo's `scripts/gates.list`;
  triggered by its own couples (`kit:checks/*.sh` plus
  `scripts/gates.list`).

## Existing sections updated

- gate-sdk/SPEC.md §check-graph — couples grammar gains the `kit:` form;
  the manifest-reader contract moves the expansion into `lib/gate.sh` so
  emitter and checker cannot desync.
- gate-sdk/SPEC.md §gen-pre-commit — emitted blocks carry expanded
  couples; regeneration duty on kit-set change stated.
- delegation-kit/SPEC.md §Layout and configuration —
  `DELEGATION_KIT_META_PATHS` documented as unioned with kit roots, the
  knob now carrying only non-kit prefixes.

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
      retired (the per-kit hand lists); nothing dangles.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed as
      debt tasks (a build-time causal gap is resolved that session, not
      deferred).
